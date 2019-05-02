//
// Created by genshen on 2018-06-09.
//

#include "capi.h"
#include "converter_c.h"

int ParseBinaryAtoms(const char *filename, unsigned int ranks, on_atom_read atom_read) {
    TypeAtom atom;

    CAtomParser_T *pParser = NewParser(filename);
    Parse(pParser, ranks); // todo rank size.
    if (HasError(pParser)) {
        return 1; // error while open file
    }
    // todo check error.
    while (ReadNext(pParser, &atom)) {
        atom_read(atom);
    }
    return 0;
}
