//
// Created by genshen on 2018-06-10.
//

#ifndef C_API_CONVERTER_C_H

#define  C_API_CONVERTER_C_H

/**
 * c header for calling c++ class
 */
typedef struct CAtomParser_T CAtomParser_T;

extern "C" {
CAtomParser_T *NewParser(const char *filename);

int HasError(CAtomParser_T *p);

int Parse(CAtomParser_T *p, int n_ranks);

int ReadNext(CAtomParser_T *p, TypeAtom *atom);
}
#endif // C_API_CONVERTER_C_H
