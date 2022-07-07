package main

import (
	"log"

	"github.com/IoIxD/YASE/core"
)

func main() {
	err := core.Run()
	if(err != nil) {
		log.Fatalln(err.Error())
	}
}