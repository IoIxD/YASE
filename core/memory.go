package core

// the fake "memory" for a project so to speak; variables, etc.

import (
	"fmt"
	"encoding/json"
)

type Project struct {
	Objects    []Object 	`json:"targets"`
	// Monitor isn't implemented because even if there WAS an editor it's useless
	Extensions []string     `json:"extensions"`
}

func (p Project) String() (returnString string) {
	returnString = "Objects: {"
	for _, v := range p.Objects {
		returnString += fmt.Sprintf(`
	"%s": {
		IsStage: %b
		CurrentCostume: %.0f
		Volume: %.0f
		LayerOrder: %.0f
		Tempo: %.0f
		VideoTransparency: %.0f
		VideoState: %s
		PositionX: %.0f
		PositionY: %.0f
		Size: %.0f
		Direction: %.0f
		Draggable: %b
		RotationStyle: %s
	}`,
		v.Name, v.IsStage, v.CurrentCostume, v.Volume, v.LayerOrder, v.Tempo,
		v.VideoTransparency, v.VideoState, v.PositionX, v.PositionY, v.Size,
		v.Direction, v.Draggable, v.RotationStyle)
	}
	returnString += "\n}\nExtenstions: ["
	for i, v := range p.Extensions {
		returnString += v
		if(len(p.Extensions) < i) {
			returnString += ", "
		}
	}
	returnString += "]"
	return
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
	IsStage 				bool   							`json:"isStage"`
	Name    				string 							`json:"name"`
	Variables  				map[string][]any             	`json:"variables"`
	Lists      				map[string][]any             	`json:"lists"`
	Broadcasts 				map[string]string 				`json:"broadcasts"`
	Blocks     				map[string]Block 	            `json:"blocks"`
	// comments aren't implemented
	CurrentCostume  		float32 						`json:"currentCostume"`
	Costumes   				[]Costume   	             	`json:"costumes"`
	Sounds     				[]Sound  	            		`json:"sounds"`
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
	AssetId          		string							`json:"assetId"`
	Name             		string							`json:"name"`
	BitmapResolution 		int 							`json:"bitmapResolution"`
	MD5              		string 							`json:"md5ext"`
	DataFormat       		string 							`json:"dataFormat"`
	RotationCenterX  		float32 						`json:"rotationCenterX"`
	RotationCenterY  		float32 						`json:"rotationCenterY"`
}

// A sound in Scratch
type Sound struct {
	AssetId         		string							`json:"assetId"`
	Name            		string							`json:"name"`
	DataFormat      		string							`json:"dataFormat"`
	Rate            		float32 						`json:"rate"`
	SampleCount     		float32 						`json:"sampleCount"`
	MD5 					string 							`json:"md5ext"`
}

// A set of instructions in Scratch
type Block struct {
	Opcode      			string 							`json:"opcode"`
	NextBlock   			string 							`json:"next"`
	ParentBlock 			string 							`json:"parent"`
	Inputs      			map[string]any 					`json:"inputs"`
	Fields      			map[string]any 					`json:"fields"`
	Shadow      			bool 							`json:"shadow"`
	TopLevel    			bool 							`json:"topLevel"`
}

func JSONToMemory(body []byte) (project *Project, err error) {
	project = &Project{}
	err = json.Unmarshal(body, project)
	if err != nil {
		return nil, err
	}

	return
}
