//
// Created by genshen on 2018-06-09.
//

#ifndef C_API_ATOM_TYPE_H

#define  C_API_ATOM_TYPE_H

#include <stdlib.h>

/**
 * basic atom type saved in binary atom file.
 * each atom in binary atom file will include those information below.
 */
typedef struct type_c_atom {
    unsigned long id; // atom id
    size_t step; // time step
    int type; // atom type
    short inter_type;
    double atom_location[3]; // atom location
    double atom_velocity[3]; // atom velocity
} TypeAtom;

#endif // C_API_ATOM_TYPE_H