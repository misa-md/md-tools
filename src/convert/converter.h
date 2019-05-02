//
// Created by genshen on 2018-06-10.
//

#ifndef C_API_CONVERTER_H

#define  C_API_CONVERTER_H

#include <fstream>
#include "atom_type.h"

typedef char byte;

class AtomParser {
public:

    // initialize input file here.
    AtomParser(const std::string &file_path);

    ~ AtomParser();

    bool parse(int ranks);

    // read information of next atom.
    bool next(TypeAtom *atom);

    bool hasError();

private:
    int processors; // the count of mpi ranks.
    int next_rank_id = 0;
    size_t atoms_in_rank = 0;
    size_t atoms_read_in_rank = 0;
    std::ifstream infile;

    // atom buffer
    int cursor = 0;
    int length = 0;
    const static int buffer_capacity = 1024;
    const static int HEADER_SIZE = 128;
    const static int _LOCAL_HEADER_SIZE = 128;
    const static int BLOCK_SIZE = buffer_capacity * sizeof(TypeAtom);
    TypeAtom buffer[buffer_capacity];

    bool has_error;

    /**
     * fill buffer.
     * @return the count of atoms/data read from file.
     */
    size_t fill();

    /**
     * read bytes array from binary atom file.
     * If current block don't have enough data. it will jump to next block to read enough dada.
     * @param buff buffer for saving data.
     * @param buff_size the data size need read in bytes.
     * @param rank_n the total MPI ranks.
     * @param rank current rank id (0 <= rank < rank_n).
     */
    void readBytes(byte *buff, size_t buff_size, int rank_n, int rank);

    bool moreInThisRank();

    /**
     * checkout to next mpi rank in data file.
     * @return if no more ranks left, false will be returned, otherwise true will be returned.
     */
    bool checkoutToNextRank();
};

#endif // C_API_CONVERTER_H