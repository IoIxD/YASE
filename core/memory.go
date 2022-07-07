package core

// the fake "memory" for a project so to speak; variables, etc.

import (
	"fmt"
	"encoding/json"
	"reflect"
)

// The Project struct is basically the root for Scratch games. All memory and...extensions? are kept here.
type Project struct {
	Objects    []*Object 	`json:"targets"`
	// Monitor isn't implemented because even if there WAS an editor it's useless
	Extensions []*string     `json:"extensions"`
}

func (p Project) String() (returnString string) {
	return fmt.Sprintf("Objects: %v,\nExtensions: %v",p.Objects,p.Extensions)
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
	IsStage 				bool   						`json:"isStage"`
	Name    				string 						`json:"name"`
	Variables  				map[string][]*any             	`json:"variables"`
	Lists      				map[string][]*any             	`json:"lists"`
	Broadcasts 				map[string]*string 			`json:"broadcasts"`
	Blocks     				map[string]*Block 	            `json:"blocks"`
	// comments aren't implemented
	CurrentCostume  		float32 						`json:"currentCostume"`
	Costumes   				[]*Costume   	             	`json:"costumes"`
	Sounds     				[]*Sound  	            		`json:"sounds"`
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

func (o Object) String() (string) {
	if(reflect.ValueOf(o).Kind() == reflect.Ptr) {
		return fmt.Sprintf("%v\n",&o)
	}
	return fmt.Sprintf(`
	"%s": {
		IsStage: %t
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
		Draggable: %t
		RotationStyle: %s
		Variables: (%d objects)
		Lists: (%d objects)
		Blocks: (%d objects)
		Broadcasts: (%d objects) 
	}
	`,
	o.Name, o.IsStage, o.CurrentCostume, o.Volume, o.LayerOrder, o.Tempo,
	o.VideoTransparency, o.VideoState, o.PositionX, o.PositionY, o.Size,
	o.Direction, o.Draggable, o.RotationStyle,
	len(o.Variables),len(o.Lists),len(o.Blocks),len(o.Broadcasts))
}

// Scratch doesn't have a pointer system so it assigns random names to blocks.
// This function gives us pointers through that system.
func (o Object) Block(name string) (block *Block, ok bool) {
	block, ok = o.Blocks[name]
	return 
}

// But we want the pointers stored in the struct so we don't have to do that function each time.
// So here's a function for populating the pointers in a Block struct based on the pointer.
// (and anything else that the json unmarshal function doesn't put in)
func (o Object) Populate(parent *Project) {
	for _, v := range o.Blocks {
		if p, ok := o.Block(v.ParentBlockPointer); ok {
			v.ParentBlock = p
		}
		if n, ok := o.Block(v.NextBlockPointer); ok {
			v.NextBlock = n
		}
	}
}

// Self explanatory
type Variable struct {
	Name  string
	Value any
}

func (v Variable) String() (string) {
	return fmt.Sprintf(`{Name: "%s", Value: %v}`,v.Name,v.Value)
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

func (l List) String() (string) {
	return fmt.Sprintf(`"%s": %v`,l.Name,l.Value)
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

func (o Costume) String() (string) {
	return fmt.Sprintf(`
	"%s": {
		AssetId: %v
		BitmapResolution: %v
		MD5: %v
		DataFormat: %v
		RotationCenterX: %v
		RotationCenterY: %v
	}
	`,
	o.Name, o.AssetId, o.BitmapResolution, o.MD5, o.DataFormat, o.RotationCenterX, o.RotationCenterY)
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

func (o Sound) String() (string) {
	return fmt.Sprintf(`
	"%s": {
		AssetId: %v
		DataFormat: %v
		Rate: %v
		SampleCount: %v
		MD5: %v
	}
	`,
	o.Name, o.AssetId, o.DataFormat, o.Rate, o.SampleCount, o.MD5)
}


// A set of instructions in Scratch
type Block struct {
	Opcode      			string 							`json:"opcode"`
	NextBlockPointer   		string 							`json:"next"`
	NextBlock 				*Block
	ParentBlockPointer 		string 							`json:"parent"`
	ParentBlock 			*Block
	Inputs      			map[string]*any 				`json:"inputs"`
	Fields      			map[string]*any 				`json:"fields"`
	Shadow      			bool 							`json:"shadow"`
	TopLevel    			bool 							`json:"topLevel"`
}

func (b *Block) String() (string) {
	return fmt.Sprintf(`
	{
		Opcode: %v
		NextBlock: %v
		ParentBlock: %v
		Inputs: (%v objects)
		Fields: (%v objects)
		Shadow: %v
		TopLevel: %v
	}
	`,
	b.Opcode, &b.NextBlock, &b.ParentBlock, len(b.Inputs), len(b.Fields), b.Shadow, b.TopLevel)
}

func JSONToMemory(body []byte) (project *Project, err error) {
	project = &Project{}
	err = json.Unmarshal(body, project)
	if err != nil {
		return nil, err
	}
	for _, v := range project.Objects {
		v.Populate(project)
	}
	return
}
