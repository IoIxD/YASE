package main

import (
	"github.com/IoIxD/YASE/core"
)

func main() {
	err := core.Run()
	if(err != nil) {
		println(err.Error())
	}
}