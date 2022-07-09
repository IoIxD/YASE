package core

// Implementations of the functions in the Events section

import (
	"fmt"

	"github.com/IoIxD/YASE/core/syscall"
)

// Prettier, changable names for event related opcodes.

const (
	opcodeFlagClicked 			= "event_whenflagclicked"
	opcodeKeyClicked  			= "event_whenkeypressed"
	opcodeObjectClicked  		= "event_whenthisspriteclicked"
	opcodeBackdropSwitched  	= "event_whenbackdropswitchesto"
	opcodeVariableGreaterThen 	= "event_whengreaterthan"
	opcodeBroadcastRecieved  	= "event_whenbroadcastreceived"
	opcodeStageClicked 			= "event_whenstageclicked"
	opcodeBroadcast 			= "event_broadcast"
)

// First, the channels that events should listen to.
var whenFlagClicked 		chan string
var whenStageClicked  		chan string
var whenKeyClicked 			chan string
var whenObjectClicked 		chan *Object
var whenBackdropSwitched 	chan string
var whenVariableGreaterThen chan string
var whenBroadcastRecieved 	chan string

// Then, the listeners that take listen on those channels
func (b *Block) OnFlagClick() {
	for {
		// On compiled ports, there should be no flag button. Just start automatically.
		if(syscall.CompiledFor == "static") {
			<- whenFlagClicked
		}
		b.NextBlock.Execute()
	}
}

func (b *Block) OnStageClick() {
	for {
		<-whenStageClicked
		// there's only ever one stage, so if we recieve anything then it's safe to assume we should continue.
		b.NextBlock.Execute()
	}
}

func (b *Block) OnKeyClick() {
	for {
		key := <-whenKeyClicked
		if(key == b.Fields["NEXT_OPTION"].Name) {
			b.NextBlock.Execute()
		}
	}
}

func (b *Block) OnObjectClick() {
	for {
		object := <-whenObjectClicked
		if(object == b.Parent) {
			b.NextBlock.Execute()
		}
	}
}

func (b *Block) OnBackdropSwitch() {
	for {
		backdrop := <-whenBackdropSwitched
		if(backdrop == b.Fields["BACKDROP"].Name) {
			b.NextBlock.Execute()
		}
	}
}

func (b *Block) OnVariableGreaterThen() {
	for {
		variable := <-whenVariableGreaterThen
		if(variable == b.Fields["WHENGREATERTHANMENU"].Name) {
			b.NextBlock.Execute()
		}
	}
}

func (b *Block) OnBroadcast() {
	for {
		broadcast := <-whenBroadcastRecieved
		if(broadcast == b.Fields["BROADCAST_OPTION"].Value) {
			b.NextBlock.Execute()
		}
	}
}

func (b *Block) Broadcast() {
	fmt.Println("fields")
	fmt.Println(b.Fields)
	if(b.Fields["BROADCAST_INPUT"] == nil) {
		return
	} 
	whenBroadcastRecieved <- b.Fields["BROADCAST_INPUT"].Values[2]
	
}