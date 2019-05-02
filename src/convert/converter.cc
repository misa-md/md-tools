//
// Created by genshen on 2018-06-10.
//

#include <iostream>
#include "atom_type.h"
#include "converter.h"

AtomParser::AtomParser(const std::string &file_path)
        : next_rank_id(0), atoms_in_rank(0), atoms_read_in_rank(0),
        infile(file_path, std::ios::in | std::ios::binary) {
    if (!infile.good()) { // open file failed.
        has_error = true;
    }
}

AtomParser::~AtomParser() {
    if (infile.good()) { // close file.
        infile.close();
    }
}

bool AtomParser::hasError() {
    return has_error;
}

bool AtomParser::parse(int ranks) {
    this->processors = ranks;
    return true;
}

bool AtomParser::next(TypeAtom *atom) {
//    {id:12, step:16, type:0, inter_type:2};
    if (cursor >= length) { // data not in buffer, we need read data from file.
        if (!moreInThisRank()) { // if no more atoms in this rank, we should switch to next rank.
            bool status = checkoutToNextRank();
            next_rank_id++; // move to next rank.
            atoms_read_in_rank = 0; // reset read counter
            if (!status) { // no more rank (end of file).
                return false;
            }
        }
        size_t size = fill(); // read data for this rank (there must be some data).
        atoms_read_in_rank += size; // add tp read counter.
    }

    // now, data is in buffer.
    std::copy(buffer + cursor, buffer + cursor + 1, atom);
    cursor++;
    return true;
}

size_t AtomParser::fill() {
    size_t atom_left_in_rank = atoms_in_rank - atoms_read_in_rank;
    size_t atom_read = atom_left_in_rank > buffer_capacity ? buffer_capacity : atom_left_in_rank;
    readBytes((byte *) buffer, atom_read * sizeof(TypeAtom), processors, next_rank_id - 1); // use current rank id

    // reset cursor.
    cursor = 0;
    length = atom_read;
    return atom_read;
}

void AtomParser::readBytes(byte *buff, size_t buff_size, int rank_n, int rank) {
    long begin = infile.tellg();
    long H = HEADER_SIZE + _LOCAL_HEADER_SIZE * rank_n;
    long k = (begin - H) / BLOCK_SIZE / rank_n; // now it is on the (k+1)th blocks.
    long next_rank_block = H + (rank_n * (k + 1) + rank) * BLOCK_SIZE;
    long next_block = H + (rank_n * k + rank + 1) * BLOCK_SIZE;

    long left_this_block = next_block - begin > buff_size ? buff_size : next_block - begin;
#ifdef DEBUG
    std::cout << "k:" << k << " next_rank_block: " << next_rank_block << " next_block: " << next_block
              << " left_this_block: " << left_this_block << ".\n";

    std::cout << "(1) read from " << infile.tellg() << " of " << left_this_block << " bytes. \n";
#endif

    infile.read(buff, left_this_block);
    int gap = sizeof(TypeAtom) - (BLOCK_SIZE * (rank + 1)) % sizeof(TypeAtom);
    if (left_this_block < buff_size) { // read more
        infile.seekg(next_rank_block, std::ios::beg); // goto next block.
#ifdef DEBUG
        std::cout << "(2) read from " << infile.tellg() << " of " << buff_size - left_this_block
                  << " bytes " << +".\n";
#endif
        infile.read(buff + left_this_block, gap); // todo fixme bug: can not combine.
        infile.read(buff + left_this_block + gap, buff_size - left_this_block - gap);
        infile.read(buff + left_this_block, buff_size - left_this_block);
    }
    if (infile.tellg() == next_block) { // automatic goes to next rank block.
        infile.seekg(next_rank_block, std::ios::beg);
    }
#ifdef DEBUG
    std::cout << "\n";
#endif
}

bool AtomParser::moreInThisRank() {
#ifdef DEBUG
    std::cout << atoms_read_in_rank << ":" << atoms_in_rank << "\n";
#endif
    if (atoms_read_in_rank < atoms_in_rank) {
        return true;
    }
    return false;
}

bool AtomParser::checkoutToNextRank() {
    if (next_rank_id >= processors) { // no more rank.
        return false;
    }
    if (!infile.good()) { // check file.
        has_error = true;
        return false;
    }

    int &current_rank_id = next_rank_id;
    // read header first.
    infile.seekg(HEADER_SIZE + _LOCAL_HEADER_SIZE * current_rank_id, std::ios::beg);
    struct {
        size_t atoms_count;
    } local_header;

    infile.read((byte *) &local_header, sizeof(local_header));
    atoms_in_rank = local_header.atoms_count; // set atom count in this rank

    // switch to atom storage body.
    infile.seekg(HEADER_SIZE + _LOCAL_HEADER_SIZE * processors + BLOCK_SIZE * current_rank_id, std::ios::beg);
    // todo check out of range(file length)
    return true;
}