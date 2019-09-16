//
// Created by genshen on 2018-06-10.
// Convert from cpp to c on 2019-08-03
//

#include <stdlib.h>
#include <string.h>
#include "converter.h"

#define DEFAULT_BLOCK_SIZE 1024
const unsigned int buffer_capacity = 1024;
const unsigned int HEADER_SIZE = 128;
const unsigned int _LOCAL_HEADER_SIZE = 128;
const unsigned int BLOCK_SIZE = DEFAULT_BLOCK_SIZE * sizeof(TypeAtom); // block size in bytes

// private functions here.
/**
 * check if there are more atoms in this rank.
 * \param p_parser pointer of parser
 * \return true for having more atoms, false for otherwise
 */
bool moreInThisRank(AtomParser *p_parser);

/**
 * when finished reading of one rank, we can switch to next mpi rank in data file.
 * \param p_parser pointer of parser
 * \return if no more ranks left, false will be returned, otherwise true will be returned.
 */
bool switchToNextRank(AtomParser *p_parser);

/**
 * fill buffer
 * \param p_parser pointer of parser
 * \return the count of atoms read from file.
 */
size_t fill(AtomParser *p_parser);

/**
  * read bytes array from binary atom file.
  ** If current block don't have enough data. it will jump to next block to read enough dada.
  * @param infile file pointer to be read from.
  * @param buff buffer for saving data.
  * @param buff_size the data size need read in bytes.
  * @param rank_n the total MPI ranks.
  * \param rank current rank id starting from 0 (available rank: 0 <= rank < rank_n).
  */
void readBytes(FILE *infile, byte *buff, size_t buff_size, int rank_n, int rank);

AtomParser MakeParser(unsigned int processors, const char *file_path) {
    AtomParser parser = {
            .processors = processors,
            .infile = NULL,
            .next_rank_id = 0,
            .atoms_in_rank = 0,
            .atoms_read_in_rank = 0,
            .cursor = 0,
            .length = 0,
            .buffer=NULL,
            .has_error = false,
    };

    // open file
    FILE *p_file = fopen(file_path, "rb");
    if (p_file == NULL) {
        parser.has_error = true;
        return parser;
    }
    parser.infile = p_file;

    // todo read global header to get block size.
    parser.block_size = DEFAULT_BLOCK_SIZE;
    // create reader buffer (make make buffer size equals to block size)
    parser.buffer_size = DEFAULT_BLOCK_SIZE;
    parser.buffer = (TypeAtom *) malloc(parser.buffer_size * sizeof(TypeAtom));
    return parser;
}

void OnParsingDone(AtomParser *p_parser) {
    // close file
    fclose(p_parser->infile);
    // release buffer memory
    free(p_parser->buffer);
}

bool next(AtomParser *p_parser, TypeAtom *atom) {
//    {id:12, step:16, type:0, inter_type:2};
    if (p_parser->cursor >= p_parser->length) { // no more data in buffer, we need read data from file.
        if (!moreInThisRank(p_parser)) { // if no more atoms in this rank, we should switch to next rank.
            bool status = switchToNextRank(p_parser);
            p_parser->next_rank_id++; // move to next rank.
            p_parser->atoms_read_in_rank = 0; // reset read counter
            if (!status) { // no more rank (end of file).
                return false;
            }
        }
        size_t atom_read = fill(p_parser); // read data for this rank (there must be some data).
        // reset cursor and buffer length.
        p_parser->cursor = 0;
        p_parser->length = atom_read;
        // add tp read counter.
        p_parser->atoms_read_in_rank += atom_read;
    }

    // now, data is in buffer.
    memcpy(atom, p_parser->buffer + p_parser->cursor, 1 * sizeof(TypeAtom));
    p_parser->cursor++;
    return true;
}

bool moreInThisRank(AtomParser *p_parser) {
#ifdef DEBUG
    printf("%ld:%ld\n", p_parser->atoms_read_in_rank, p_parser->atoms_in_rank);
#endif
    if (p_parser->atoms_read_in_rank < p_parser->atoms_in_rank) {
        return true;
    }
    return false;
}

bool switchToNextRank(AtomParser *p_parser) {
    if (p_parser->next_rank_id >= p_parser->processors) { // no more rank.
        return false;
    }

    int current_rank_id = p_parser->next_rank_id;
    // read header first.
    fseek(p_parser->infile, HEADER_SIZE + _LOCAL_HEADER_SIZE * current_rank_id, SEEK_SET);
    struct {
        size_t atoms_count;
    } local_header;

    fread((byte *) &local_header, sizeof(local_header), 1, p_parser->infile);
    p_parser->atoms_in_rank = local_header.atoms_count; // set atom count in this rank

    // switch to atom storage body.
    fseek(p_parser->infile,
          HEADER_SIZE + _LOCAL_HEADER_SIZE * p_parser->processors + BLOCK_SIZE * current_rank_id,
          SEEK_SET);
    // todo check out of range(file length)
    return true;
}

size_t fill(AtomParser *p_parser) {
    size_t atom_left_in_rank = p_parser->atoms_in_rank - p_parser->atoms_read_in_rank;
    size_t atom_read = atom_left_in_rank > p_parser->buffer_size ? p_parser->buffer_size : atom_left_in_rank;
    readBytes(p_parser->infile, (byte *) p_parser->buffer,
              atom_read * sizeof(TypeAtom), p_parser->processors,
              p_parser->next_rank_id - 1); // use current rank id
    return atom_read;
}

void readBytes(FILE *infile, byte *buff, size_t buff_size, int rank_n, int rank) {
    const long begin = ftell(infile);
    const long H = HEADER_SIZE + _LOCAL_HEADER_SIZE * rank_n; // header size(global + local)
    const long k = (begin - H) / BLOCK_SIZE / rank_n; // now it is on the (k+1)th blocks.
    // bytes index of next block in the same rank
    const long next_rank_block = H + (rank_n * (k + 1) + rank) * BLOCK_SIZE;
    // bytes index of next global block.
    const long next_block = H + (rank_n * k + rank + 1) * BLOCK_SIZE;

    // bytes will be read in current block.
    const long left_this_block = next_block - begin > buff_size ? buff_size : next_block - begin;
#ifdef DEBUG
    printf("k: %ld, next_rank_block: %ld, next_block: %ld, left_this_block: %ld ",
            k, next_rank_block, next_block, left_this_block);
    printf("(1) read from %ld of %ld bytes.\n", ftell(infile), left_this_block);
#endif

    if (left_this_block != 0) {
        fread(buff, left_this_block, 1, infile);
    }
    // it may split a struct as two part, gap is the size in bytes left in next block.
    const int gap = sizeof(TypeAtom) - (BLOCK_SIZE * (rank + 1)) % sizeof(TypeAtom);
    if (left_this_block < buff_size) { // need to read more
        fseek(infile, next_rank_block, SEEK_SET); // goto next block.
#ifdef DEBUG
        printf("(2) read from %ld of %ld bytes.\n", ftell(infile), buff_size - left_this_block);
#endif
        if (gap != 0) {
            fread(buff + left_this_block, gap, 1, infile);
//            infile->read(buff + left_this_block, gap); // todo fixme bug: can not combine.
        }
        fread(buff + left_this_block + gap, buff_size - left_this_block - gap, 1, infile);
//        fread(read(buff + left_this_block, buff_size - left_this_block, 1, infile);
    }
    if (ftell(infile) == next_block) { // automatic goes to next rank block.
        fseek(infile, next_rank_block, SEEK_SET);
    }
#ifdef DEBUG
    printf("\n");
#endif
}
