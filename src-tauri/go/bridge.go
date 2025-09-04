package main

/*
#cgo CFLAGS: -I${SRCDIR}/../gen
#include "spacebar_core.h"
*/
import "C"

//export go_add_two
func go_add_two(x C.int) C.int {
	return C.rust_add_one(x) + 1
}

func main() {}
