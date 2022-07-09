// +build !js

package syscall

// File for any functions that "aren't implemented" on js but are implemented on other platforms, so we call those other functions
// when not compiling for js.

import (
	"os"
)

// while we're here, we have this variable here that sets the type of program we're compiling for, so that we can avoid having to split functions 
// into different files so much

const (
	CompiledFor = "static"
)

func ReadFile(name string) ([]byte, error) {
	return os.ReadFile(name)
}