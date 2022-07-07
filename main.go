package main

import (
	"fmt"
)

func main() {
	project, err := GetProject()
	if(err != nil) {
		fmt.Println(err)
		return
	}
	fmt.Println(project)
}