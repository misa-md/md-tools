package main

import "C"
import (
)

//export ReadMinioFile
func ReadMinioFile(path *C.char) (err *C.char) {

	return C.CString("")
}

//export StopClientWrapper
func StopClientWrapper(handlesPtr uintptr) *C.char {
	return C.CString("")
}

func main() {}
