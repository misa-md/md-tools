//
// Created by genshen on 2018-06-10.
//

#include "converter.h"

#include "converter_c.h"
// #include "atom_type.h"


struct CAtomParser_T : AtomParser {
    CAtomParser_T(const std::string &file_path) : AtomParser(file_path) {}

    ~CAtomParser_T() {}
};

CAtomParser_T *NewParser(const char *filename) {
    auto p = new CAtomParser_T(std::string(filename));
    if (p->hasError()) {
        return nullptr;
    }
    return p;
};

int HasError(CAtomParser_T *p) {
    return p->hasError();
}

int Parse(CAtomParser_T *p, int n_ranks) {
    return p->parse(n_ranks);
}

int ReadNext(CAtomParser_T *p, TypeAtom *atom) {
    return p->next(atom); // todo bool to int.
};
