package main

// the fake "memory" for a project so to speak; variables, etc.

import (
	"encoding/json"
	"fmt"
	"image"
)

type Project struct {
	Objects []*Object
	// Monitor isn't implemented because even if there WAS an editor it's useless
	Extensions []string
}

type ProjectJSON struct {
	Objects    []ObjectJSON `json:"targets"`
	Extensions []string     `json:"extensions"`
}

// Any values that can be stored in memory
type AcceptableValue interface {
	int
	string
	float32
	float64
}

// What Scratch calls a "sprite"; we call it an object because that makes
// more sense to a seasoned programmer
type Object struct {
	IsStage    bool
	Name       string
	Variables  map[string]Variable
	Lists      map[string]List
	Broadcasts map[string]string // no struct needed
	Blocks     map[string]Block

	Costumes             []Costume
	Sounds               []Sound
	Volume               float32
	LayerOrder           float32
	Tempo                float32
	VideoTransparency    float32
	VideoState           bool
	TextToSpeechLanguage interface{} // todo: what is this
	Position             Position
	Size                 int16
	Direction            int16
	Draggable            bool
	RotationStyle        string
}

type Position struct {
	X, Y int16
}

// the json version(s) of this struct that we unmarshal from the json file,
// and then convert to a safer struct
type ObjectJSON struct {
	IsStage 				bool   							`json:"isStage"`
	Name    				string 							`json:"name"`
	Variables  				map[string][]any             	`json:"variables"`
	Lists      				map[string][]any             	`json:"lists"`
	Broadcasts 				map[string]string 				`json:"broadcasts"`
	Blocks     				map[string]BlockJSON            `json:"blocks"`
	// comments aren't implemented
	CurrentCostume  		float32 						`json:"currentCostume"`
	Costumes   				[]CostumeJSON                	`json:"costumes"`
	Sounds     				[]SoundJSON              		`json:"sounds"`
	Volume              	float32       					`json:"volume"`
	LayerOrder           	float32       					`json:"layerOrder"`
	Tempo                	float32       					`json:"tempo"`
	VideoTransparency    	float32       					`json:"videoTransparency"`
	VideoState           	string        					`json:"videoState"`
	TextToSpeechLanguage 	interface{} 					`json:"textToSpeechLanguage"`
	PositionX            	float32       					`json:"x"`
	PositionY            	float32       					`json:"y"`
	Size                 	float32       					`json:"size"`
	Direction            	float32       					`json:"direction"`
	Draggable            	bool        					`json:"draggable"`
	RotationStyle        	string      					`json:"rotationStyle"`
}

// Self explanatory
type Variable struct {
	Name  string
	Value any
}

func (Variable) NewSet() map[string]Variable {
	return make(map[string]Variable)
}

// Scratch's version of arrays (todo: do we really need this?
// or can we just convert these to regular arrays?)

type List struct {
	Name  string
	Value []any
}

// A graphic in Scratch
type Costume struct {
	Asset           image.Image
	Name            string
	Resolution      int
	MD5             string
	Format          string
	RotationCenterX float32
	RotationCenterY float32
}

type CostumeJSON struct {
	AssetId          string
	Name             string
	BitmapResolution int
	MD5              string
	DataFormat       string
	RotationCenterX  float32
	RotationCenterY  float32
}

// A sound in Scratch
type Sound struct {
	Asset       []byte
	Name        string
	DataFormat  string
	Rate        float32
	SampleCount float32
	MD5         string
}

type SoundJSON struct {
	AssetId         string
	Name            string
	DataFormat      string
	Rate            float32
	SampleCount     float32
	MD5 			float32
}

// A set of instructions in Scratch
type Block struct {
	Opcode      string
	NextBlock   *Block
	ParentBlock *Block
	Inputs      []*Variable
	Fields      []*Variable
	Shadow      bool
	TopLevel    bool
	// position values aren't accounted for obvious reasons
}

// A struct for the json version of this that is usually translated to a
// proper block
type BlockJSON struct {
	Opcode      string `json:"opcode"`
	NextBlock   string `json:"next"`
	ParentBlock string `json:"parent"`
	Inputs      map[string]any `json:"inputs"`
	Fields      map[string]any `json:"fields"`
	Shadow      bool `json:"shadow"`
	TopLevel    bool `json:"topLevel"`
}

func JSONToMemory(body []byte) (project *Project, err error) {
	project = &Project{}


	// first, 
	projectJSON := &ProjectJSON{}
	err = json.Unmarshal(body, projectJSON)
	if err != nil {
		return nil, err
	}

	fmt.Println(projectJSON)

	return
}
