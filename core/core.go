package core

// Core functions that one might use for generally just running a Scratch application.

import (
	"fmt"
	"errors"
)

// Function for start execution of a project.
// todo: config struct
func Run() (err error) {
	// Get the project information
	project, err := GetProject()
	if(err != nil) {
		return err
	}
	// Go through all the blocks in the project and execute all the top level ones.
	for _, o := range project.Objects {
		for _, b := range o.Blocks {
			if(b.TopLevel) {
				err := b.Execute()
				if(err != nil) {
					fmt.Println(err)
				}
			}
		}
	}
	return nil
}

// Function for executing a given instruction.
func (b *Block) Execute() (error) {
	name := b.Opcode
	switch(name) {
		default: return errors.New(fmt.Sprintf("Unimplemented opcode %v",name))
	}
	return nil
}