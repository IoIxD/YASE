package core

// Core functions that one might use for generally just running a Scratch application.

import (
	"fmt"
	"errors"
	"os"
	"syscall"
	"os/signal"
)

var quitSignal 	chan bool


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
					println(err.Error())
				}
			}
		}
	}
	quitSignal := make(chan os.Signal)
	signal.Notify(quitSignal, syscall.SIGINT, syscall.SIGTERM)
	<-quitSignal
	return nil
}


// Function for executing a given instruction.
func (b *Block) Execute() (error) {
	name := b.Opcode
	switch(name) {
		case opcodeFlagClicked:					go b.OnFlagClick()
		case opcodeKeyClicked: 					go b.OnKeyClick()
		case opcodeObjectClicked:  				go b.OnObjectClick()
		case opcodeBackdropSwitched:  			go b.OnBackdropSwitch()
		case opcodeVariableGreaterThen: 		go b.OnVariableGreaterThen()
		case opcodeBroadcastRecieved: 			go b.OnBroadcast()
		case opcodeStageClicked:  				go b.OnStageClick()
		case opcodeBroadcast: 					b.Broadcast()
		default: 								return errors.New(fmt.Sprintf("Unimplemented opcode %v",name))
	}
	return nil
}