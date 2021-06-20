//
// Created by genshen on 2018-06-09.
//

/**
 * this file is the interface for calling C function by Go.
 */
#ifndef C_API_H

#define  C_API_H

#include "atom_type.h"

/**
 * callback function used by Go if there is some atoms read.
 */
typedef int (*on_atom_read)(void *callback_obj, const TypeAtom atom);


/**
 * create a parser from binary file and MPI ranks
 * \param filename file path of binary file
 * \param ranks MPI ranks for running the MD simulation
 * \return the parser pointer
 */
void *make_parse(const char *filename, unsigned int ranks);

/**
 * read next atom via parser.
 * \param _obj parser object
 * \return 0 for ok, 1 for "no atoms left"
 */
int read_next_atom(void *_obj, TypeAtom *atom);

/**
 * close parser and free memory
 */
void close_parser(void *_obj);

#endif // C_API_H