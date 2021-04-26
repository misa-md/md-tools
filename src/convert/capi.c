//
// Created by genshen on 2018-06-09.
//

#include <stdio.h>
#include <stdlib.h>
#include "capi.h"
#include "converter.h"


void *make_parse(const char *filename, unsigned int ranks) {
    AtomParser parser = MakeParser(ranks, filename);
    if (parser.has_error) {
        printf("Error, initialize parser failed with file `%s` and ranks %d.\n", filename, ranks);
        return NULL; // error while opening file
    }
    AtomParser *p_parser = malloc(sizeof(AtomParser));
    *p_parser = parser;
    return p_parser;
}

int read_next_atom(void *_obj, TypeAtom *atom) {
    AtomParser *parser = (AtomParser *) _obj;
    if (next(parser, atom)) {
        return 0;
    }
    return 1;
}

void close_parser(void *_obj) {
    AtomParser *parser = (AtomParser *) _obj;
    // release buffer memory and close file.
    OnParsingDone(parser);
    free(parser);
}
