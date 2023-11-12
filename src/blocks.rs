///
/// This module is specifically for deserialization of blocks in the .json
/// files into their respective types, in such a way that is Rust-idomatic,
/// deals away with unused data, and is generally nicer to work with.
///
/// It should be noted that the following rules are applied when translating
/// these blocks:
/// - Blocks that are the same but with a different argument are represented by an enum that leads to the respective types. For example, "goto <option>" and "goto "<x, y>" are one "goto" enum with a "Pos" and "Option" option.
/// - Since there's no logic here, Scratch's "sprite globals" i.e. "x position" are also represented by structs, and its up to you to resolve them. This also goes for functions with no arguments such as "next costume". They're not just enums because...
/// - **Every single struct has a 'prev' and 'next' field, even if it doesn't show up in the documentation!** These represent the previous and next block, respectively.
/// - Blocks that are considered redundant or unused or marked as "UnusedOpcode" structs to avoid confusion. These are blocks that have a value that isn't even used, and it just...goes to the next block and uses that value.
// what
use lazy_static::lazy_static;
use proc::block_derive;
use regex::Regex;
use serde::de::{value::StringDeserializer, Visitor};
#[allow(dead_code)]
use serde::{de, Deserialize, Deserializer};
use serde_json::{Map, Value as SerdeValue};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Pointer},
};

pub use crate::block_defs::{
    control::*, custom::*, data::*, events::*, look::*, motion::*, operators::*, sensing::*,
    sound::*,
};
use crate::block_names::*;
use std::fmt::Display;

// Any block that has a prev/next field.
pub trait Block {
    fn prev(&self) -> Option<String>;
    fn next(&self) -> Option<String>;
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl Display for dyn Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // :3
        let mut lol: String = String::new();
        let mut f2 = Formatter::new(&mut lol);
        self.debug_fmt(&mut f2)?;

        lol = format!("WHAT{}", lol);

        f.write_str(lol.as_str())
    }
}

/// Either a number or a String, the latter signifying a pointer to another block.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub enum BlockType {
    // Motion blocks
    Move(Move),
    RotateLeft(RotateLeft),
    RotateRight(RotateRight),
    /// goto a MovementOption, or an x and y coordinate.
    Goto(Goto),
    /// glide to a MovementOption, or an x and y coordinate.
    Glide(Glide),
    /// point to a MovementOption or in a direction
    Point(Point),
    ChangeX(ChangeX),
    SetX(SetX),
    ChangeY(ChangeY),
    SetY(SetY),
    IfOnEdgeBounce(IfOnEdgeBounce),
    SetRotationStyle(SetRotationStyle),
    XPosition(XPosition),
    YPosition(YPosition),
    Direction(Direction),
    // Look blocks
    Say(Say),
    SayForever(SayForever),
    Think(Think),
    ThinkForever(ThinkForever),
    SwitchCostume(SwitchCostume),
    NextCostume(NextCostume),
    SwitchBackdrop(SwitchBackdrop),
    SwitchBackdropAndWait(SwitchBackdropAndWait),
    NextBackdrop(NextBackdrop),
    ChangeSize(ChangeSize),
    SetSize(SetSize),
    ClearGraphicEffects(ClearGraphicEffects),
    ShowSprite(ShowSprite),
    HideSprite(HideSprite),
    HideAllSprites(HideAllSprites),
    GotoLayer(GotoLayer),
    ChangeLayer(ChangeLayer),
    Costume(Costume),
    Backdrop(Backdrop),
    Size(Size),
    // Sound blocks
    PlaySound(PlaySound),
    PlaySoundUntilDone(PlaySoundUntilDone),
    StartSound(StartSound),
    StopAllSounds(StopAllSounds),
    ChangeEffectBy(ChangeEffectBy),
    SetEffectTo(SetEffectTo),
    ClearSoundEffects(ClearSoundEffects),
    ChangeVolumeBy(ChangeVolumeBy),
    SetVolumeTo(SetVolumeTo),
    Volume(Volume),
    // Event blocks
    WhenGreenFlagClicked(WhenGreenFlagClicked),
    WhenKeyPressed(WhenKeyPressed),
    WhenSpriteClicked(WhenSpriteClicked),
    WhenStageClicked(WhenStageClicked),
    WhenBackdropSwitchesTo(WhenBackdropSwitchesTo),
    WhenOptionGreaterThen(WhenOptionGreaterThen),
    WhenIRecieveBroadcast(WhenIRecieveBroadcast),
    Broadcast(Broadcast),
    BroadcastAndWait(BroadcastAndWait),
    // Control blocks
    WaitSeconds(WaitSeconds),
    Repeat(Repeat),
    Forever(Forever),
    IfThen(IfThen),
    IfThenElse(IfThenElse),
    WaitUntil(WaitUntil),
    RepeatUntil(RepeatUntil),
    StopAll(StopAll),
    WhenIStartAsAClone(WhenIStartAsAClone),
    CreateCloneOf(CreateCloneOf),
    DeleteClone(DeleteClone),
    // Sensing blocks
    Touching(Touching),
    TouchingMenu(TouchingMenu),
    TouchingColor(TouchingColor),
    ColorTouchingColor(ColorTouchingColor),
    DistanceTo(DistanceTo),
    Answer(Answer),
    KeyPressed(KeyPressed),
    MouseDown(MouseDown),
    MouseX(MouseX),
    MouseY(MouseY),
    DraggableOption(DraggableOption),
    SetDragMode(SetDragMode),
    Loudness(Loudness),
    Timer(Timer),
    ResetTimer(ResetTimer),
    BackdropOf(BackdropOf),
    CurrentTime(CurrentTime),
    DaysSince2000(DaysSince2000),
    Username(Username),
    // Operator blocks
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Divide(Divide),
    PickRandom(PickRandom),
    GreaterThen(GreaterThen),
    LesserThen(LesserThen),
    EqualTo(EqualTo),
    And(And),
    Or(Or),
    Not(Not),
    Join(Join),
    LetterOf(LetterOf),
    LengthOf(LengthOf),
    Contains(Contains),
    Modulo(Modulo),
    Round(Round),
    Absolute(Absolute),
    SoundEffectsMenu(SoundEffectsMenu),
    SoundSoundsMenu(SoundSoundsMenu),
    PointTowardsMenu(PointTowardsMenu),

    DataGetVariable(DataGetVariable),
    DataSetVariableTo(DataSetVariableTo),
    DataChangeVariableBy(DataChangeVariableBy),
    DataShowVariable(DataShowVariable),
    DataHideVariable(DataHideVariable),
    DataListContents(DataListContents),
    DataListIndexAll(DataListIndexAll),
    DataListIndexAllRandom(DataListIndexAllRandom),
    DataAddToList(DataAddToList),
    DataDeleteOfList(DataDeleteOfList),
    DataDeleteAllOfList(DataDeleteAllOfList),
    DataInsertAtList(DataInsertAtList),
    DataReplaceItemOfList(DataReplaceItemOfList),
    DataItemOfList(DataItemOfList),
    DataLengthOfList(DataLengthOfList),
    DataListContainsItem(DataListContainsItem),
    ShowList(DataShowList),
    HideList(DataHideList),

    ProceduresCall(ProceduresCall),
    ProceduresDeclaration(ProceduresDeclaration),
    ProceduresDefinition(ProceduresDefinition),
    ProceduresPrototype(ProceduresPrototype),

    /// some opcodes are straight up unused or redundant and should be labelled as such.
    UnusedOpcode(UnusedOpcode),
    /// some aren't implemented, but in release mode we need to ignore them.
    InvalidOpcode(InvalidOpcode),
    /// some aren't connected to anything.
    Stray,
}

lazy_static! {
    static ref DUP_REGEX: Regex = Regex::new("(.*?)\\((.*?) \\{(.*?)\\}\\)").unwrap();
    static ref SOME_REGEX: Regex = Regex::new("(Some|String)\\((.*?)\\)").unwrap();
    static ref NUM_REGEX: Regex = Regex::new("Number\\((.*?)\\)").unwrap();
    static ref PREV_REGEX: Regex = Regex::new("prev: (.*?)(,|\\s})").unwrap();
    static ref NEXT_REGEX: Regex = Regex::new("next: (.*?)(,|\\s})").unwrap();
}

/// We syphen the information from BlockType's debug trait and remove what we don't want, because due to the size of the enum, doing this is quicker then doing it properly.
impl Display for BlockType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // :3
        let mut lol: String = String::new();
        let mut f2 = Formatter::new(&mut lol);
        Debug::fmt(self, &mut f2)?;

        lol = DUP_REGEX.replace_all(&lol, "$1 {\"$3\"}").to_string();
        lol = SOME_REGEX.replace_all(&lol, "$2").to_string();
        lol = NUM_REGEX
            .replace_all(&lol, "$2")
            .to_string()
            .replace("\"", "");

        let mut lol2 = NEXT_REGEX.replace_all(&lol, "}").to_string();
        lol2 = PREV_REGEX.replace_all(&lol2, "").to_string();

        if !lol2.contains("{  }") {
            lol = lol2;
        }
        lol = lol.replace("  ", "");

        f.write_str(lol.as_str())
    }
}

// macro for implemeneting "from" based on given f.
#[macro_export]
macro_rules! from_fn_from_map {
    ($structname:ty, {$($name:tt => $result:ident,)*}) => {
        impl $structname {
            pub fn from(val: Option<Value>) -> Option<$structname> {
                match val {
                    Some(a) => {
                        match a {
                            Value::String(a) => match a.as_str() {
                                $(
                                    $name => Some(Self::$result),
                                )*
                                _ => {
                                    #[cfg(debug_assertions)]
                                    panic!("invalid $structname: {}",a);
                                    #[cfg(not(debug_assertions))]
                                    None
                                },
                            },
                            _ => {
                                #[cfg(debug_assertions)]
                                panic!("invalid type given for $structname, expected string");
                                #[cfg(not(debug_assertions))]
                                None
                            },
                        }
                    }
                    None => None,
                }
            }
        }
    }
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct UnusedOpcode {
    name: String,
}

#[block_derive]
#[derive(Debug, Clone)]
pub struct InvalidOpcode {
    pub(crate) name: String,
}

// Deserializiation implementation
// numbers only regex
lazy_static! {
    static ref NUMBERS_ONLY_REGEX: Regex = Regex::new(r"[^0-9]").unwrap();
}
impl<'de> Visitor<'de> for BlockType {
    type Value = SerdeValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid value that the serde package can handle")
    }
}

enum HashOrVec {
    Hash(HashMap<String, SerdeValue>),
    Vec(Vec<SerdeValue>),
}
struct HashOrVecVisitor;
impl<'de> Visitor<'de> for HashOrVecVisitor {
    type Value = HashOrVec;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map or a sequence")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut hashmap: HashMap<String, SerdeValue> = HashMap::new();
        #[allow(irrefutable_let_patterns)] // we handle breaking ourselves.
        while let k = map.next_entry() {
            match k {
                Ok(a) => match a {
                    Some(a) => {
                        hashmap.insert(a.0, a.1);
                    }
                    None => {
                        break;
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(HashOrVec::Hash(hashmap))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut vec: Vec<SerdeValue> = Vec::new();
        #[allow(irrefutable_let_patterns)] // we handle breaking ourselves.
        while let a = seq.next_element() {
            match a {
                Ok(a) => match a {
                    Some(a) => vec.push(a),
                    None => {
                        break;
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(HashOrVec::Vec(vec))
    }
}

fn idk<'a>(
    f: (&'a String, &'a SerdeValue),
    id: usize,
) -> Result<(&'a String, Value), serde_json::Error> {
    let b = f.1.get(id);
    let block = match b {
        Some(a) => {
            let bl;
            if a.is_array() {
                bl = a.as_array().unwrap().get(1).unwrap();
            } else {
                bl = a
            };
            if bl.is_null() {
                Ok((f.0, Value::Null))
            } else if bl.is_u64() {
                Ok((f.0, Value::Number(bl.as_u64().unwrap() as f64)))
            } else if bl.is_i64() {
                Ok((f.0, Value::Number(bl.as_i64().unwrap() as f64)))
            } else if bl.is_f64() {
                Ok((f.0, Value::Number(bl.as_f64().unwrap())))
            } else {
                let st = bl.as_str().unwrap();
                if st == "" {
                    return Ok((f.0, Value::Null));
                }
                if NUMBERS_ONLY_REGEX.is_match(st) {
                    Ok((f.0, Value::String(st.to_string())))
                } else {
                    match st.parse() {
                        Ok(a) => Ok((f.0, Value::Number(a))),
                        Err(_) => {
                            return Err(format!("could not format {} into a number", st))
                                .map_err(de::Error::custom)
                        }
                    }
                }
            }
        }
        None => {
            return Err(format!("what")).map_err(de::Error::custom);
        } // we should never reach this
    };
    block
}

/// A representation of what we'd expect blocks to have.
struct RawBlock {
    opcode: String,
    next: Option<String>,
    parent: Option<String>,
    inputs: HashMap<String, Value>,
    fields: HashMap<String, Value>,
    params: HashMap<String, Value>,
    hash: HashMap<String, SerdeValue>,
}

impl<'de> Visitor<'de> for RawBlock {
    type Value = SerdeValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid value that the serde package can handle")
    }
}

impl<'de> Deserialize<'de> for RawBlock {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hash = match d.deserialize_any(HashOrVecVisitor) {
            Ok(a) => match a {
                HashOrVec::Hash(a) => a,
                HashOrVec::Vec(a) => {
                    let mut count = 0;
                    a.into_iter()
                        .map(|f| {
                            count += 1;
                            (count.to_string(), f)
                        })
                        .collect()
                }
            },
            Err(err) => {
                return Err(err);
            }
        };
        match hash.get_key_value("opcode") {
            Some(a) => {
                let name: &String = match a.1 {
                    SerdeValue::String(a) => a,
                    _ => {
                        return Err(format!("not a string: {}", a.1)).map_err(de::Error::custom);
                    }
                };
                //
                // Each block has three parts.
                // The first value is always a number and signifies whether there's a "shadow".
                // - 1 (SameBlockShadow): unobscured "shadow": the second value is a block
                // - 2 (BlockNoShadow): no shadow: the second value is a reference to a block
                // - 3 (DiffBlockShadow): obscured shadow: the second value is a reference to a block and the third is a "shadow"
                // a "shadow" is something that's only important to the visual editor; its the value that the user dragged a block over.
                // we don't care about this.

                // inputs is never null only empty
                let (_, inputs) = hash.get_key_value("inputs").unwrap();
                let inputs = inputs.as_object().unwrap();

                // same with fields
                let (_, fields) = hash.get_key_value("fields").unwrap();
                let fields = fields.as_object().unwrap();

                let params;
                if let Some((_, a)) = hash.get_key_value("params") {
                    params = Some(a.as_object().unwrap());
                } else {
                    params = None;
                }

                let inputs: HashMap<String, Value> = inputs
                    .into_iter()
                    .map(|f| idk(f, 1))
                    .map(|f| f.unwrap())
                    .filter(|f| f.1 != Value::Null)
                    .map(|f| (f.0.clone(), f.1))
                    .collect();

                let fields: HashMap<String, Value> = fields
                    .into_iter()
                    .map(|f| idk(f, 0))
                    .map(|f| f.unwrap())
                    .filter(|f| f.1 != Value::Null)
                    .map(|f| (f.0.clone(), f.1))
                    .collect();

                let params = match params {
                    Some(a) => a
                        .into_iter()
                        .map(|f| idk(f, 0))
                        .map(|f| f.unwrap())
                        .filter(|f| f.1 != Value::Null)
                        .map(|f| (f.0.clone(), f.1))
                        .collect(),

                    None => HashMap::new(),
                };

                let prev = match hash.get_key_value("parent") {
                    Some(a) => Some(a.to_owned().1.to_string()),
                    None => None,
                };
                let next = match hash.get_key_value("next") {
                    Some(a) => Some(a.to_owned().1.to_string()),
                    None => None,
                };

                Ok(RawBlock {
                    opcode: name.clone(),
                    next: next.clone(),
                    parent: prev.clone(),
                    inputs,
                    fields,
                    params: params,
                    hash,
                })
            }
            None => {
                return Err("no opcode").map_err(de::Error::custom);
            }
        }
    }
}

impl<'de> Deserialize<'de> for BlockType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = RawBlock::deserialize(d)?;

        //let name = raw.name;
        let inputs = raw.inputs;
        let fields = raw.fields;
        let params = raw.params;

        let val1 = inputs.get("0").cloned();
        let val2 = inputs.get("1").cloned();
        let val3 = inputs.get("2").cloned();
        let val4 = inputs.get("3").cloned();
        let val5 = inputs.get("4").cloned();
        let field1 = fields.get("0").cloned();
        let field2 = fields.get("1").cloned();
        let field3 = fields.get("2").cloned();
        let field4 = fields.get("3").cloned();
        let field5 = fields.get("4").cloned();

        let prev = raw.parent;
        let next = raw.next;

        let block_type: Result<BlockType, <D as Deserializer>::Error> = match raw.opcode.as_str() {
            MOTION_MOVE => Ok(BlockType::Move(Move {
                steps: val1,
                prev,
                next,
            })),
            MOTION_GOTO_XY => Ok(BlockType::Goto(Goto::Pos(GotoPos {
                x: val1,
                y: val2,
                prev,
                next,
            }))),
            MOTION_GOTO_MENU => Ok(BlockType::Goto(Goto::Option(GotoOption {
                option: MovementOption::from(val1),
                prev,
                next,
            }))),
            MOTION_TURN_LEFT => Ok(BlockType::RotateLeft(RotateLeft {
                degrees: val1,
                prev,
                next,
            })),
            MOTION_TURN_RIGHT => Ok(BlockType::RotateRight(RotateRight {
                degrees: val1,
                prev,
                next,
            })),
            // Unused: Mouse. It's always mouse.
            MOTION_POINT_MENU => Ok(BlockType::Point(Point::Towards(PointOption {
                option: Some(Value::String(String::from("_mouse_"))),
                prev,
                next,
            }))),
            MOTION_POINT_DIRECTION => Ok(BlockType::Point(Point::Direction(PointDirection {
                x: val1,
                y: val2,
                prev,
                next,
            }))),
            MOTION_POINT_TOWARDS => Ok(BlockType::Point(Point::Towards(PointOption {
                option: val1,
                prev,
                next,
            }))),
            MOTION_GLIDE_SECONDS_TO_XY => Ok(BlockType::Glide(Glide::Pos(GlidePos {
                x: val1,
                y: val2,
                prev,
                next,
            }))),
            MOTION_GLIDE_TO_MENU => Ok(BlockType::Glide(Glide::Option(GlideOption {
                option: MovementOption::from(field1),
                prev,
                next,
            }))),
            MOTION_IF_ON_EDGE_BOUNCE => {
                Ok(BlockType::IfOnEdgeBounce(IfOnEdgeBounce { prev, next }))
            }
            MOTION_SET_ROTATION_STYLE => Ok(BlockType::SetRotationStyle(SetRotationStyle {
                style: RotationStyle::from(val1),
                prev,
                next,
            })),
            MOTION_CHANGE_X_BY => Ok(BlockType::ChangeX(ChangeX {
                x: val1,
                prev,
                next,
            })),
            MOTION_SET_X => Ok(BlockType::SetX(SetX {
                x: val1,
                prev,
                next,
            })),
            MOTION_CHANGE_Y_BY => Ok(BlockType::ChangeY(ChangeY {
                y: val1,
                prev,
                next,
            })),
            MOTION_SET_Y => Ok(BlockType::SetY(SetY {
                y: val1,
                prev,
                next,
            })),
            MOTION_XPOSITION => Ok(BlockType::XPosition(XPosition { prev, next })),
            MOTION_YPOSITION => Ok(BlockType::YPosition(YPosition { prev, next })),
            MOTION_DIRECTION => Ok(BlockType::Direction(Direction { prev, next })),
            // presumed unused
            MOTION_SCROLL_RIGHT => {
                todo!()
            }
            MOTION_SCROLL_UP => {
                todo!()
            }
            MOTION_ALIGN_SCENE => {
                todo!()
            }
            MOTION_XSCROLL => {
                todo!()
            }
            MOTION_YSCROLL => {
                todo!()
            }
            LOOKS_SAY => Ok(BlockType::SayForever(SayForever {
                message: val1,
                prev,
                next,
            })),
            LOOKS_SAY_FOR_SECS => Ok(BlockType::Say(Say {
                message: val1,
                secs: val2,
                prev,
                next,
            })),
            LOOKS_THINK => Ok(BlockType::ThinkForever(ThinkForever {
                message: val1,
                prev,
                next,
            })),
            LOOKS_THINK_FOR_SECS => Ok(BlockType::Think(Think {
                message: val1,
                secs: val2,
                prev,
                next,
            })),
            LOOKS_SHOW => Ok(BlockType::ShowSprite(ShowSprite { prev, next })),
            LOOKS_HIDE => Ok(BlockType::HideSprite(HideSprite { prev, next })),
            LOOKS_HIDE_ALL_SPRITES => Ok(BlockType::HideAllSprites(HideAllSprites { prev, next })),
            LOOKS_SWITCH_COSTUME_TO => Ok(BlockType::SwitchCostume(SwitchCostume {
                costume: val1,
                prev,
                next,
            })),
            LOOKS_SWITCH_BACKDROP_TO => Ok(BlockType::SwitchBackdrop(SwitchBackdrop {
                backdrop: val1,
                prev,
                next,
            })),
            LOOKS_SWITCH_BACKDROP_TO_AND_WAIT => {
                Ok(BlockType::SwitchBackdropAndWait(SwitchBackdropAndWait {
                    backdrop: val1,
                    prev,
                    next,
                }))
            }
            LOOKS_NEXT_COSTUME => Ok(BlockType::NextCostume(NextCostume { prev, next })),
            LOOKS_NEXT_BACKDROP => Ok(BlockType::NextBackdrop(NextBackdrop { prev, next })),
            LOOKS_CHANGE_EFFECT_BY => Ok(BlockType::ChangeEffectBy(ChangeEffectBy {
                effect: val1,
                units: val2,
                prev,
                next,
            })),
            LOOKS_SET_EFFECT_TO => Ok(BlockType::SetEffectTo(SetEffectTo {
                effect: val1,
                percentage: val2,
                prev,
                next,
            })),
            LOOKS_CLEAR_GRAPHICS_EFFECTS => {
                Ok(BlockType::ClearGraphicEffects(ClearGraphicEffects {
                    prev,
                    next,
                }))
            }
            LOOKS_CHANGE_SIZE_BY => Ok(BlockType::ChangeSize(ChangeSize {
                units: val1,
                prev,
                next,
            })),
            LOOKS_SET_SIZE_TO => Ok(BlockType::SetSize(SetSize {
                percentage: val1,
                prev,
                next,
            })),
            LOOKS_CHANGE_STRETCH_BY => {
                todo!()
            }
            LOOKS_SET_STRETCH_TO => {
                todo!()
            }
            LOOKS_GOTO_FRONT_BACK => Ok(BlockType::GotoLayer(GotoLayer {
                option: LayerOption::from(val1),
                prev,
                next,
            })),
            LOOKS_GO_FORWARD_BACKWARD_LAYERS => Ok(BlockType::ChangeLayer(ChangeLayer {
                direction: LayerDirection::from(val1),
                by: val2,
                prev,
                next,
            })),
            LOOKS_SIZE => Ok(BlockType::Size(Size { prev, next })),
            LOOKS_COSTUME => Ok(BlockType::Costume(Costume::WithName(field1))),
            LOOKS_COSTUME_NUMBER_NAME => match field1 {
                Some(Value::String(a)) => match a.as_str() {
                    "number" => Ok(BlockType::Costume(Costume::ByNumber(CostumeByNumber {
                        prev,
                        next,
                    }))),
                    "name" => Ok(BlockType::Costume(Costume::ByName(CostumeByName {
                        prev,
                        next,
                    }))),
                    _ => {
                        return Err(format!("invalid option given for costume number/name"))
                            .map_err(de::Error::custom);
                    }
                },
                _ => {
                    return Err(format!("no option given for costume number/name"))
                        .map_err(de::Error::custom);
                }
            },
            LOOKS_BACKDROP => Ok(BlockType::Backdrop(Backdrop::WithName(field1))),
            LOOKS_BACKDROP_NUMBER_NAME => match field1 {
                Some(Value::String(a)) => match a.as_str() {
                    "number" => Ok(BlockType::Backdrop(Backdrop::ByNumber(BackdropByNumber {
                        prev,
                        next,
                    }))),
                    "name" => Ok(BlockType::Backdrop(Backdrop::ByName(BackdropByName {
                        prev,
                        next,
                    }))),
                    _ => {
                        return Err(format!("invalid option given for backdrop number/name"))
                            .map_err(de::Error::custom);
                    }
                },
                _ => {
                    return Err(format!("no option given for backdrop number/name"))
                        .map_err(de::Error::custom);
                }
            },
            SOUND_PLAY => Ok(BlockType::PlaySound(PlaySound {
                sound: val1,
                prev,
                next,
            })),
            SOUND_PLAY_UNTIL_DONE => Ok(BlockType::PlaySoundUntilDone(PlaySoundUntilDone {
                sound: val1,
                prev,
                next,
            })),
            SOUND_STOP_ALL_SOUNDS => Ok(BlockType::StopAllSounds(StopAllSounds { prev, next })),
            SOUND_SET_EFFECT_TO => Ok(BlockType::SetEffectTo(SetEffectTo {
                effect: val1,
                percentage: val2,
                prev,
                next,
            })),
            SOUND_CHANGE_EFFECT_BY => Ok(BlockType::ChangeEffectBy(ChangeEffectBy {
                effect: val1,
                units: val2,
                prev,
                next,
            })),
            SOUND_CLEAR_EFFECTS => Ok(BlockType::ClearSoundEffects(ClearSoundEffects {
                prev,
                next,
            })),
            SOUND_SET_VOLUME_TO => Ok(BlockType::SetVolumeTo(SetVolumeTo {
                percentage: val1,
                prev,
                next,
            })),
            SOUND_CHANGE_VOLUME_BY => Ok(BlockType::ChangeVolumeBy(ChangeVolumeBy {
                units: val1,
                prev,
                next,
            })),
            SOUND_VOLUME => Ok(BlockType::Volume(Volume { prev, next })),
            EVENT_WHEN_TOUCHING_OBJECT => {
                todo!()
            }
            EVENT_BROADCAST => Ok(BlockType::Broadcast(Broadcast {
                broadcast: val1,
                prev,
                next,
            })),
            EVENT_BROADCAST_AND_WAIT => Ok(BlockType::BroadcastAndWait(BroadcastAndWait {
                broadcast: val1,
                prev,
                next,
            })),
            EVENT_WHEN_GREATER_THAN => {
                Ok(BlockType::WhenOptionGreaterThen(WhenOptionGreaterThen {
                    option: EventOption::from(val1),
                    by: val2,
                    prev,
                    next,
                }))
            }
            EVENT_WHEN_FLAG_CLICKED => Ok(BlockType::WhenGreenFlagClicked(WhenGreenFlagClicked {
                prev,
                next,
            })),
            EVENT_WHEN_KEY_PRESSED => Ok(BlockType::WhenKeyPressed(WhenKeyPressed {
                key: Key::from(val1),
                prev,
                next,
            })),
            EVENT_WHEN_THIS_SPRITECLICKED => Ok(BlockType::WhenSpriteClicked(WhenSpriteClicked {
                prev,
                next,
            })),
            EVENT_WHEN_STAGE_CLICKED => {
                Ok(BlockType::WhenStageClicked(WhenStageClicked { prev, next }))
            }
            EVENT_WHEN_BACKDROP_SWITCHESTO => {
                Ok(BlockType::WhenBackdropSwitchesTo(WhenBackdropSwitchesTo {
                    backdrop: val1,
                    prev,
                    next,
                }))
            }
            EVENT_WHEN_BROADCAST_RECEIVED => {
                Ok(BlockType::WhenIRecieveBroadcast(WhenIRecieveBroadcast {
                    broadcast: field1,
                    prev,
                    next,
                }))
            }

            CONTROL_REPEAT => Ok(BlockType::Repeat(Repeat {
                units: val1,
                prev,
                next,
            })),
            CONTROL_REPEAT_UNTIL => Ok(BlockType::RepeatUntil(RepeatUntil {
                condition: val1,
                prev,
                next,
            })),
            CONTROL_WHILE => Ok(BlockType::RepeatUntil(RepeatUntil {
                condition: val1,
                prev,
                next,
            })),
            CONTROL_FOR_EACH => {
                todo!() // i don't see this in the scratch part picker what?
            }
            CONTROL_FOREVER => Ok(BlockType::Forever(Forever { prev, next })),
            CONTROL_WAIT => Ok(BlockType::WaitSeconds(WaitSeconds {
                seconds: val1,
                prev,
                next,
            })),
            CONTROL_WAIT_UNTIL => Ok(BlockType::WaitUntil(WaitUntil {
                condition: val1,
                prev,
                next,
            })),
            CONTROL_IF => Ok(BlockType::IfThen(IfThen {
                condition: val1,
                then: val2,
                prev,
                next,
            })),
            CONTROL_IF_ELSE => Ok(BlockType::IfThenElse(IfThenElse {
                condition: val1,
                then: val2,
                otherwise: val3,
                prev,
                next,
            })),
            CONTROL_STOP => Ok(BlockType::StopAll(StopAll { prev, next })),
            CONTROL_CREATE_CLONE_OF => Ok(BlockType::CreateCloneOf(CreateCloneOf {
                of: SpriteOption::from(val1),
                prev,
                next,
            })),
            CONTROL_DELETE_THIS_CLONE => Ok(BlockType::DeleteClone(DeleteClone { prev, next })),
            /*CONTROL_GET_COUNTER => {
                todo!()
            }
            CONTROL_INCREMENT_COUNTER => {
                todo!()
            }
            CONTROL_CLEAR_COUNTER => {
                todo!()
            }
            CONTROL_ALL_AT_ONCE => {
                todo!()
            }
            CONTROL_START_AS_CLONE => {
                todo!()
            }*/
            SENSING_TOUCHING_OBJECT_MENU => Ok(BlockType::TouchingMenu(TouchingMenu {
                touching: SensingOption::from(field1),
                prev,
                next,
            })),
            SENSING_TOUCHING_OBJECT => Ok(BlockType::Touching(Touching {
                touching: val1,
                prev,
                next,
            })),
            SENSING_TOUCHING_COLOR => Ok(BlockType::TouchingColor(TouchingColor {
                color: val1,
                prev,
                next,
            })),
            SENSING_COLOR_IS_TOUCHING_COLOR => {
                Ok(BlockType::ColorTouchingColor(ColorTouchingColor {
                    color1: val1,
                    color2: val2,
                    prev,
                    next,
                }))
            }
            SENSING_DISTANCE_TO => Ok(BlockType::DistanceTo(DistanceTo {
                to: SensingOption::from(val1),
                prev,
                next,
            })),
            SENSING_TIMER => Ok(BlockType::Timer(Timer { prev, next })),
            SENSING_RESET_TIMER => Ok(BlockType::ResetTimer(ResetTimer { prev, next })),
            // uses fields.
            SENSING_OF => {
                todo!()
            }
            SENSING_MOUSE_X => Ok(BlockType::MouseX(MouseX { prev, next })),
            SENSING_MOUSE_Y => Ok(BlockType::MouseY(MouseY { prev, next })),
            SENSING_SET_DRAG_MODE => Ok(BlockType::SetDragMode(SetDragMode {
                option: DraggableOption::from(val1),
                prev,
                next,
            })),
            SENSING_MOUSE_DOWN => Ok(BlockType::MouseDown(MouseDown { prev, next })),
            SENSING_KEY_PRESSED => Ok(BlockType::KeyPressed(KeyPressed {
                key: Key::from(val1),
                prev,
                next,
            })),
            SENSING_CURRENT => Ok(BlockType::CurrentTime(CurrentTime {
                option: CurrentTimeOption::from(val1),
                prev,
                next,
            })),
            SENSING_DAYS_SINCE_2000 => Ok(BlockType::DaysSince2000(DaysSince2000 { prev, next })),
            SENSING_LOUDNESS => Ok(BlockType::Loudness(Loudness { prev, next })),
            SENSING_LOUD => {
                todo!() // What?
            }
            SENSING_ASK_AND_WAIT => {
                todo!()
            }
            SENSING_ANSWER => Ok(BlockType::Answer(Answer { prev, next })),
            SENSING_USERNAME => Ok(BlockType::Username(Username { prev, next })),
            SENSING_USER_ID => {
                todo!()
            }

            OPERATOR_ADD => Ok(BlockType::Add(Add {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_SUBTRACT => Ok(BlockType::Sub(Sub {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_MULTIPLY => Ok(BlockType::Mul(Mul {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_DIVIDE => Ok(BlockType::Divide(Divide {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_LESSER_THEN => Ok(BlockType::LesserThen(LesserThen {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_EQUALS => Ok(BlockType::EqualTo(EqualTo {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_GREATER_THEN => Ok(BlockType::GreaterThen(GreaterThen {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_AND => Ok(BlockType::And(And {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_OR => Ok(BlockType::Or(Or {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_NOT => Ok(BlockType::Not(Not {
                a: val1,
                prev,
                next,
            })),
            OPERATOR_RANDOM => Ok(BlockType::PickRandom(PickRandom {
                min: val1,
                max: val2,
                prev,
                next,
            })),
            OPERATOR_JOIN => Ok(BlockType::Join(Join {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_LETTER_OF => Ok(BlockType::LetterOf(LetterOf {
                index: val1,
                a: val2,
                prev,
                next,
            })),
            OPERATOR_LENGTH => Ok(BlockType::LengthOf(LengthOf {
                a: val1,
                prev,
                next,
            })),
            OPERATOR_CONTAINS => Ok(BlockType::Contains(Contains {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_MOD => Ok(BlockType::Modulo(Modulo {
                a: val1,
                b: val2,
                prev,
                next,
            })),
            OPERATOR_ROUND => Ok(BlockType::Round(Round {
                a: val1,
                prev,
                next,
            })),
            OPERATOR_MATHOP => Ok(BlockType::Absolute(Absolute {
                a: val1,
                prev,
                next,
            })),
            SOUND_SOUNDS_MENU => Ok(BlockType::SoundSoundsMenu(SoundSoundsMenu {
                option: val1,
                prev,
                next,
            })),

            SOUND_EFFECTS_MENU => Ok(BlockType::SoundEffectsMenu(SoundEffectsMenu {
                option: SoundEffect::from(val1),
                prev,
                next,
            })),
            DATA_VARIABLE => Ok(BlockType::DataGetVariable(DataGetVariable {
                variable: params.get("VARIABLE").unwrap().clone(),
                prev,
                next,
            })),

            DATA_SET_VARIABLE_TO => Ok(BlockType::DataSetVariableTo(DataSetVariableTo {
                variable: fields.get("VARIABLE").unwrap().clone(),
                value: inputs.get("VALUE").unwrap().clone(),
                prev,
                next,
            })),

            DATA_CHANGE_VARIABLE_BY => Ok(BlockType::DataChangeVariableBy(DataChangeVariableBy {
                variable: fields.get("VARIABLE").unwrap().clone(),
                value: inputs.get("VALUE").unwrap().clone(),
                prev,
                next,
            })),

            DATA_HIDE_VARIABLE => Ok(BlockType::DataHideVariable(DataHideVariable {
                variable: fields.get("VARIABLE").unwrap().clone(),
                prev,
                next,
            })),

            DATA_SHOW_VARIABLE => Ok(BlockType::DataShowVariable(DataShowVariable {
                variable: fields.get("VARIABLE").unwrap().clone(),
                prev,
                next,
            })),

            DATA_LIST_COTNENTS => Ok(BlockType::DataListContents(DataListContents {
                variable: fields.get("VARIABLE").unwrap().clone(),
                prev,
                next,
            })),

            DATA_ADD_TO_LIST => Ok(BlockType::DataAddToList(DataAddToList {
                item: inputs.get("ITEM").unwrap().clone(),
                list: fields.get("LIST").unwrap().clone(),
                prev,
                next,
            })),

            DATA_DELETE_OF_LIST => Ok(BlockType::DataDeleteOfList(DataDeleteOfList {
                item: inputs.get("INDEX").unwrap().clone(),
                list: fields.get("LIST").unwrap().clone(),
                prev,
                next,
            })),

            DATA_DELETE_ALL_OF_LIST => Ok(BlockType::DataDeleteAllOfList(DataDeleteAllOfList {
                list: fields.get("LIST").unwrap().clone(),
                prev,
                next,
            })),

            DATA_INSERT_AT_LIST => Ok(BlockType::DataInsertAtList(DataInsertAtList {
                item: inputs.get("ITEM").unwrap().clone(),
                list: fields.get("LIST").unwrap().clone(),
                index: inputs.get("INDEX").unwrap().clone(),
                prev,
                next,
            })),

            DATA_REPLACE_ITEM_OF_LIST => {
                Ok(BlockType::DataReplaceItemOfList(DataReplaceItemOfList {
                    item: inputs.get("ITEM").unwrap().clone(),
                    list: fields.get("LIST").unwrap().clone(),
                    index: inputs.get("INDEX").unwrap().clone(),
                    prev,
                    next,
                }))
            }

            DATA_ITEM_OF_LIST => Ok(BlockType::DataItemOfList(DataItemOfList {
                list: fields.get("LIST").unwrap().clone(),
                index: inputs.get("INDEX").unwrap().clone(),
                prev,
                next,
            })),

            DATA_LENGTH_OF_LIST => Ok(BlockType::DataLengthOfList(DataLengthOfList {
                list: fields.get("LIST").unwrap().clone(),
                prev,
                next,
            })),

            DATA_LIST_CONTAINS_ITEM => Ok(BlockType::DataListContainsItem(DataListContainsItem {
                input: inputs.get("ITEM").unwrap().clone(),
                list: fields.get("LIST").unwrap().clone(),
                prev,
                next,
            })),

            PROCEDURES_CALL => Ok(BlockType::ProceduresCall(ProceduresCall { prev, next })),
            PROCEDURES_DECLARATION => Ok(BlockType::ProceduresDeclaration(ProceduresDeclaration {
                prev,
                next,
            })),
            PROCEDURES_DEFINITION => Ok(BlockType::ProceduresDefinition(ProceduresDefinition {
                block: match inputs.get("custom_block").unwrap() {
                    Value::Number(_) => todo!(),
                    Value::String(a) => a.clone(),
                    Value::Null => todo!(),
                },
                prev,
                next,
            })),
            PROCEDURES_PROTOTYPE => Ok(BlockType::ProceduresPrototype(ProceduresPrototype {
                mutation: raw
                    .hash
                    .get("mutation")
                    .unwrap()
                    .deserialize_map(MutationVisitor)
                    .unwrap(),
                prev,
                next,
            })),

            // unused opcodes
            MOTION_GOTO | SOUNDS_BEATS_MENU | MOTION_GLIDE_TO => {
                Ok(BlockType::UnusedOpcode(UnusedOpcode {
                    name: raw.opcode.to_string(),
                    prev,
                    next,
                }))
            }

            _ => {
                #[cfg(debug_assertions)]
                return Err(format!("invalid opcode {}", raw.opcode)).map_err(de::Error::custom);
                #[cfg(not(debug_assertions))]
                Ok(BlockType::InvalidOpcode(InvalidOpcode {
                    name: raw.opcode.to_string(),
                    prev,
                    next,
                }))
            }
        };
        let block_type = block_type;
        block_type
    }
}
