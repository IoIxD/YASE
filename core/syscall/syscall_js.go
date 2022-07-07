// +build js

package assembly

// File for any functions that "aren't implemented" on js but we can replicate them with js function calls, defined in index.html

import (
	"syscall/js"
	"strings"
)

func ReadFile(name string) ([]byte, error) {
	// Call a function on the Javascript side to fetch the contents of this.
	val := js.Global().Call("ReadFile",name)
	// Golang's ValueToBytes function doesn't like what this function returns.
	// So fuck it we'll just transfer the string to go ourselves.
	str := val.String()
	str = strings.Replace(str, "\\v","\\n",999)
	str = strings.Replace(str, "\\0"," ",999)
	return []byte(str), nil // if any errors shows up then golang automatically panics them to the javascript console.
}