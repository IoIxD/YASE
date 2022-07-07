// +build !js

package assembly

// File for any functions that "aren't implemented" on js but are implemented on other platforms, so we call those other functions
// when not compiling for js.

import (
	"os"
)

func ReadFile(name string) ([]byte, error) {
	return os.ReadFile(name)
}