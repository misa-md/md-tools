//
// Created by genshen on 2018-06-09.
//

/**
 * this file is the interface for calling C function by Go.
 */
#ifndef C_API_H

#define  C_API_H

#include "atom_type.h"
#include "converter.h"

/**
 * callback function used by Go if there is some atoms read.
 */
typedef int (*on_atom_read)(void *callback_obj, const TypeAtom atom);

/**
 * start to parse binary atom file.
 * @param filename file path of binary atom file.
 * @param atom_read callback function.
 * @return if all success, 0 will be returned, otherwise -1 will be returned.
 */
int ParseBinaryAtoms(const char *filename, unsigned int ranks, void *callback_obj, on_atom_read atom_read);

#endif // C_API_H