//
// Created by genshen on 2018-06-10.
//

#ifndef C_API_CONVERTER_H

#define  C_API_CONVERTER_H

#include <stdio.h>
#include <stdbool.h>
#include "atom_type.h"

typedef char byte;

typedef struct _parser {
    // constance, which will be set in function MakeParser.
    unsigned int processors; // the count of mpi ranks.
    unsigned long block_size; // block size of atoms.
    unsigned long buffer_size; // buffer size of atoms.
    FILE *infile;

    // variable
    unsigned int next_rank_id;
    size_t atoms_in_rank; // the atoms in current rank
    size_t atoms_read_in_rank; // atoms have read in this rank
    unsigned long cursor;
    unsigned long length; // length read in buffer.
    // atom buffer
    TypeAtom *buffer;

    // error: false: no error, true: has error
    bool has_error;
} AtomParser;

/**
 * create a new parser.
 * \param processors MPI ranks to run the md simulation.
 * \param file_path file path of binary output file.
 * \return parser.
 */
AtomParser MakeParser(unsigned int processors, const char *file_path);

/**
 * call when finishing parsing to release resources.
 * \param p_parser
 */
void OnParsingDone(AtomParser *p_parser);

/**
 * get next atom from buffer or file.
 * \param p_parser pointer of parser
 * \param atom atom pointer to store information of next atom.
 * \return error code: true for ok, false for having error.
 */
bool next(AtomParser *p_parser, TypeAtom *atom);

#endif // C_API_CONVERTER_H