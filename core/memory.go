package core

// the fake "memory" for a project so to speak; variables, etc.

import (
	"fmt"
	"encoding/json"
	"reflect"
	"errors"
)

// The Project struct is basically the root for Scratch games. All memory and...extensions? are kept here.
type Project struct {
	Objects    				[]*Object 						`json:"targets"`
	// Monitor isn't implemented because even if there WAS an editor it's useless
	Extensions 				[]*string     					`json:"extensions"`
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
	IsStage 				bool   							`json:"isStage"`
	Name    				string 							`json:"name"`
	Variables  				map[string]*Variable          	`json:"variables"`
	Lists      				map[string]*List             	`json:"lists"`
	Broadcasts 				map[string]*string 				`json:"broadcasts"`
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
// (SIDE NOTE: no this can't be a UnmarshalJSON function it would need to be attached to a block object 
// and we need to search through all the other blocks in the object.)
func (o Object) Populate(parent *Project) {
	for _, v := range o.Blocks {
		if p, ok := o.Block(v.ParentBlockPointer); ok {
			v.ParentBlock = p
		}
		if n, ok := o.Block(v.NextBlockPointer); ok {
			v.NextBlock = n
		}
		v.Parent = &o
		if(v.Fields != nil) {
			fmt.Println(v.Fields)
		}
	}
}

// Self explanatory...somewhat.
// Scratch's variables are actually lists that can contain any object.
// But the values we need are usually, mostly predictable, so we use an interface
// for this and attach functions for getting the parts one might look for. 
type Variable struct {
	Name string
	Value any
}

func (v *Variable) UnmarshalJSON(buf []byte) error {
	temp := []interface{}{&v.Name,&v.Value}
	if err := json.Unmarshal(buf, &temp); err != nil {
		return err
	}
	if len(temp) != 2 {
		return errors.New("Wrong number of fields were unmarshaled to Variable type")
	}
	return nil
}

func (v Variable) String() (string) {
	return fmt.Sprintf(`{Name: "%s", Value: %v}`,v.Name,v.Value)
}

func NewVariableSet() map[string]Variable {
	return make(map[string]Variable)
}

// Scratch's version of arrays (todo: do we really need this?
// or can we just convert these to regular arrays?)

type List struct {
	Name  string
	Value []any
}

func (l *List) UnmarshalJSON(buf []byte) error {
	temp := []interface{}{&l.Name,&l.Value}
	if err := json.Unmarshal(buf, &temp); err != nil {
		return err
	}
	if len(temp) != 2 {
		return errors.New("Wrong number of fields were unmarshaled to List type")
	}
	return nil
}

func (l List) String() (string) {
	return fmt.Sprintf(`"%s": %v`,l.Name,l.Value)
}

// Very much more leniant then a variable, but at the cost of being weird and slow to implement.
type Field struct {
	Name  		string
	NextBlock  	string
	Value 		string
	Values 		[]string
}

func (f *Field) UnmarshalJSON(buf []byte) error {
	// before you say it
	// i have been looking at this code for three hours.
	// i CANNOT rely on json's unmarshal due to the variability
	// of a Scratch field. i HAVE to manually unmarshal this into a field
	// because every value needs to be named and converted into a string

	var tempField Field

	var temp []interface{}
	if err := json.Unmarshal(buf, &temp); err != nil {
		return err
	}

	if(len(temp) <= 0) {return nil}
	tempField.Name = fmt.Sprintf("%v",temp[0])

	normalize := func(value any) {
		switch t := value.(type) {
			case string: 
				tempField.NextBlock = value.(string)
			case any:
				troll := reflect.ValueOf(value)
				length := troll.Len();

				if(length == 1) {
					tempField.Value = fmt.Sprintf("%v",troll.Index(0));
				}
				if(length > 1) {
					for i := 0; i < length; i++ {
						tempField.Values = append(tempField.Values,fmt.Sprintf("%v",troll.Index(i)))
					}
				}
			default:
				fmt.Println(t)
		}
	}

	if(len(temp) <= 1) {return nil}
	normalize(temp[1])
	if(len(temp) <= 2) {return nil}
	normalize(temp[2])

	f = &tempField

	return nil
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
	Inputs      			map[string]*Field		 		`json:"inputs"`
	Fields      			map[string]*Field 				`json:"fields"`
	Shadow      			bool 							`json:"shadow"`
	TopLevel    			bool 							`json:"topLevel"`
	Parent  				*Object
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
