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
	isStage    bool
	name       string
	variables  map[string]Variable
	lists      map[string]List
	broadcasts map[string]string // no struct needed
	blocks     map[string]Block
	// comments aren't implemented
	// currentCostume doesn't need to be in the struct
	costumes             []Costume
	sounds               []Sound
	volume               int32
	layerOrder           int32
	tempo                int32
	videoTransparency    int32
	videoState           bool
	textToSpeechLanguage interface{} // todo: what is this
	position             Position
	size                 int16
	direction            int16
	draggable            bool
	rotationStyle        string
}

type Position struct {
	X, Y int16
}

// the json version(s) of this struct that we unmarshal from the json file,
// and then convert to a safer struct
type ObjectJSON struct {
	isStage 				bool   							`json:"isStage"`
	name    				string 							`json:"name"`
	variables  				map[string][]any             	`json:"variables"`
	lists      				map[string][]any             	`json:"lists"`
	broadcasts 				map[string]map[string]string 	`json:"broadcasts"`
	blocks     				[]BlockJSON                  	`json:"blocks"`
	costumes   				[]CostumeJSON                	`json:"costumes"`
	sounds     				[]SoundJSON              		`json:"sounds"`
	volume              	int32       					`json:"volume"`
	layerOrder           	int32       					`json:"layerOrder"`
	tempo                	int32       					`json:"tempo"`
	videoTransparency    	int32       					`json:"videoTransparency"`
	videoState           	bool        					`json:"videoState"`
	textToSpeechLanguage 	interface{} 					`json:"textToSpeechLanguage"`
	positionX            	int32       					`json:"x"`
	positionY            	int32       					`json:"y"`
	size                 	int16       					`json:"size"`
	direction            	int16       					`json:"direction"`
	draggable            	bool        					`json:"draggable"`
	rotationStyle        	string      					`json:"rotationStyle"`
}

// Self explanatory
type Variable struct {
	name  string
	value any
}

func (Variable) NewSet() map[string]Variable {
	return make(map[string]Variable)
}

// Scratch's version of arrays (todo: do we really need this?
// or can we just convert these to regular arrays?)

type List struct {
	Name  string
	value []any
}

// A graphic in Scratch
type Costume struct {
	asset           image.Image
	name            string
	resolution      int
	md5             string
	format          string
	rotationCenterX float32
	rotationCenterY float32
}

type CostumeJSON struct {
	assetId          string
	name             string
	bitmapResolution int
	md5              string
	dataFormat       string
	rotationCenterX  float32
	rotationCenterY  float32
}

// A sound in Scratch
type Sound struct {
	asset       []byte
	name        string
	dataFormat  string
	rate        int16
	sampleCount int32
	md5         string
}

type SoundJSON struct {
	assetId         string
	name            string
	dataFormat      string
	rate            int16
	sampleCount     int32
	md5 			float32
}

// A set of instructions in Scratch
type Block struct {
	opcode      string
	nextBlock   *Block
	parentBlock *Block
	inputs      []*Variable
	fields      []*Variable
	shadow      bool
	topLevel    bool
	// position values aren't accounted for obvious reasons
}

// A struct for the json version of this that is usually translated to a
// proper block
type BlockJSON struct {
	opcode      json.RawMessage `json:"opcode"`
	nextBlock   json.RawMessage `json:"next"`
	parentBlock json.RawMessage `json:"parent"`
	inputs      json.RawMessage `json:"inputs"`
	fields      json.RawMessage `json:"fields"`
	shadow      json.RawMessage `json:"shadow"`
	topLevel    json.RawMessage `json:"topLevel"`
}

func JSONToMemory(body []byte) (project *Project, err error) {
	project = &Project{}

	projectJSON := &ProjectJSON{}
	err = json.Unmarshal(body, projectJSON)
	if err != nil {
		return nil, err
	}

	fmt.Println(projectJSON)

	return
}
