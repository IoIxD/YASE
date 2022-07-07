package core

// Core functions that one might use for generally just running a Scratch application.

import (
	"fmt"
)

// todo: config struct

func Run() (err error) {
	project, err := GetProject()
	if(err != nil) {
		return err
	}
	fmt.Println(project)
	return nil

}