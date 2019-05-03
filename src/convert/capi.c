//
// Created by genshen on 2018-06-09.
//

#include <stdio.h>
#include "capi.h"
#include "converter_c.h"

int ParseBinaryAtoms(const char *filename, unsigned int ranks, void *callback_obj, on_atom_read atom_read) {
    TypeAtom atom;

    CAtomParser_T *pParser = NewParser(filename);
    if (pParser) {
        Parse(pParser, ranks); // todo rank size.
        if (HasError(pParser)) {
            return 1; // error while open file
        }
        // todo check error.
        while (ReadNext(pParser, &atom)) {
            atom_read(callback_obj, atom);
        }
    } else {
        printf("Error, file `%s` does not exists or having wrong permission.\n", filename);
    }

    clean(pParser);
    return 0;
}
