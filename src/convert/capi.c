//
// Created by genshen on 2018-06-09.
//

#include <stdio.h>
#include "capi.h"

int ParseBinaryAtoms(const char *filename, unsigned int ranks, void *callback_obj, on_atom_read atom_read) {
    AtomParser parser = MakeParser(ranks, filename);
    if (parser.has_error) {
        printf("Error, initialize parser failed with file `%s` and ranks %d.\n", filename, ranks);
        return 1; // error while open file
    }

    TypeAtom atom;
    // todo check error.
    while (next(&parser, &atom)) {
        atom_read(callback_obj, atom);
    }

    OnParsingDone(&parser);
    return 0;
}
