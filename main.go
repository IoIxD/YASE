package main

import (
	"fmt"
)

func main() {
	_, err := GetProject()
	if(err != nil) {
		fmt.Println(err)
		return
	}
}