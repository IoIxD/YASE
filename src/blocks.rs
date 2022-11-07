extern crate proc;

use std::{collections::HashMap};

use proc::block_derive;
#[allow(dead_code)]

use serde::{Deserialize, Deserializer ,de};
use serde_json::{Value as SerdeValue};
use lazy_static::lazy_static;
use regex::Regex;
use crate::block_names;

/// Enums and structs that represent certain types of blocks in Scratch.
///
/// It should be noted that the following rules are applied when translating
/// these blocks:
/// - Blocks that are the same but with a different argument are represented by an enum that leads to the respective types. For example, "goto <option>" and "goto "<x, y>" are one "goto" enum with a "Pos" and "Option" option.
/// - Since there's no logic here, Scratch's "sprite globals" i.e. "x position" are also represented by structs, and its up to you to resolve them. This also goes for functions with no arguments such as "next costume"
/// - Some blocks have special cases where things are combined. Those are documented as such.
///

/// IDs that tell us whether a block has a shadow or not, according to Scratch's deserialization code
pub enum Inputs {
    SameBlockShadow = 1,
    BlockNoShadow = 2,
    DiffBlockShadow = 3,
}

/// IDs that distinquish different primitives.
pub enum Primitives {
    MathNum = 4,
    PositiveNum = 5,
    WholeNum = 6,
    IntegerNum = 7,
    AngleNum = 8,
    ColorPicker = 9,
    Text = 10,
    Broadcast = 11,
    Var = 12,
    List = 13,
}

/// Either a number or a String, the latter signifying a pointer to another block.
#[derive(Debug,Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

#[derive(Debug,Clone)]
pub enum BlockType {
    // Motion blocks

    /// move _ steps
    Move(Move),
    /// turn counterclockwise _ degrees.
    /// counterclockwise is represented by a negative number
    RotateLeft(RotateLeft),
    /// turn clockwise _ degrees.
    RotateRight(RotateRight),
    /// goto a MovementOption, or an x and y coordinate.
    Goto(Goto),
    /// glide to a MovementOption, or an x and y coordinate.
    Glide(Glide),
    /// point to a MovementOption or in a direction
    Point(Point),
    /// change x by _
    ChangeX(ChangeX),
    /// set x to _
    SetX(SetX),
    /// change y by _
    ChangeY(ChangeY),
    /// set y to _
    SetY(SetY),
    /// if on edge, bounce
    IfOnEdgeBounce(IfOnEdgeBounce),
    /// set rotation style
    SetRotationStyle(SetRotationStyle),
    // x position
    XPosition(XPosition),
    // y position
    YPosition(YPosition),
    // direction
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

    // """menus""" god this sucks
    SoundEffectsMenu(SoundEffectsMenu),
    SoundSoundsMenu(SoundSoundsMenu),
    PointTowardsMenu(PointTowardsMenu),

    // some opcodes are straight up unused or redundant and should be labelled as such.
    UnusedOpcode(UnusedOpcode),
    InvalidOpcode(InvalidOpcode),
}

//
// Motion Blocks
//
#[block_derive]
#[derive(Debug,Clone)]
pub struct Move {
    steps: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct RotateLeft {
    degrees: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct RotateRight {
    degrees: Option<Value>
}
#[derive(Debug,Clone)]
pub enum MovementOption {
    RandomPosition,
    MousePointer,
}
impl MovementOption {
    fn from(val: Option<Value>) -> Option<MovementOption> {
        match val {
            Some(a) => {
                match a {
                    Value::String(a) => match a.as_str() {
                        "_random_" => Some(Self::RandomPosition),
                        "_mouse_" => Some(Self::MousePointer),
                        _ => {
                            #[cfg(debug_assertions)]
                            panic!("invalid movement option: {}",a);
                            #[cfg(not(debug_assertions))]
                            None
                        },
                    },
                    _ => {
                        #[cfg(debug_assertions)]
                        panic!("invalid type given for movement option, expected string");
                        #[cfg(not(debug_assertions))]
                        None
                    },
                }
            }
            None => None,
        }
    }
}



#[derive(Debug,Clone)]
pub enum Goto {
    Pos(GotoPos),
    Option(GotoOption),
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct GotoPos {
    x: Option<Value>,
    y: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct GotoOption {
    option: Option<MovementOption>,
}
#[derive(Debug,Clone)]
pub enum Glide {
    Pos(GlidePos),
    Option(GlideOption),
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct GlidePos {
    x: Option<Value>,
    y: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct GlideOption {
    option: Option<MovementOption>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct PointTowardsMenu {
    option: Option<Point>,
}
#[derive(Debug,Clone)]
pub enum Point {
    Direction(PointDirection),
    Towards(PointOption),
}

#[block_derive]
#[derive(Debug,Clone)]
pub struct PointDirection {
    x: Option<Value>,
    y: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct PointOption {
    option: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ChangeX {
    x: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SetX {
    x: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ChangeY {
    y: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SetY {
    y: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct IfOnEdgeBounce {}
#[derive(Debug,Clone)]
pub enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}
impl RotationStyle {
    fn from(val: Option<Value>) -> Option<RotationStyle> {
        match val {
            Some(a) => {
                match a {
                    Value::String(a) => match a.as_str() {
                        "left-right" => Some(Self::LeftRight),
                        "don't rotate" => Some(Self::DontRotate),
                        "all around" => Some(Self::AllAround),
                        _ => {
                            #[cfg(debug_assertions)]
                            panic!("invalid rotation style option: {}",a);
                            #[cfg(not(debug_assertions))]
                            None
                        },
                    },
                    _ => {
                        #[cfg(debug_assertions)]
                        panic!("invalid type given for rotation style, expected string");
                        #[cfg(not(debug_assertions))]
                        None
                    },
                }
            }
            None => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SetRotationStyle {
    style: Option<RotationStyle>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct XPosition {}
#[block_derive]
#[derive(Debug,Clone)]
pub struct YPosition {}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Direction {}


//
// Look blocks
//
#[block_derive]
#[derive(Debug,Clone)]
pub struct SayForever {
    message: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Say {
    message: Option<Value>,
    secs: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Think {
    message: Option<Value>,
    secs: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ThinkForever {
    message: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SwitchCostume {
    costume: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SwitchCostumeAndWait {
    costume: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct NextCostume {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct SwitchBackdrop {
    backdrop: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SwitchBackdropAndWait {
    backdrop: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct NextBackdrop {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct ChangeSize {
    units: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SetSize {
    percentage: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ClearGraphicEffects {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct ShowSprite {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct HideSprite {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct HideAllSprites {}

#[derive(Debug,Clone)]
pub enum LayerOption {
    Front,
    Back,
}
impl LayerOption {
    fn from(val: Option<Value>) -> Option<LayerOption> {
        match val {
            Some(Value::String(a)) => {
                match a.as_str() {
                    "front" => Some(Self::Front),
                    "back" => Some(Self::Back),
                    _ => {
                        #[cfg(debug_assertions)]
                        panic!("invalid layer option: {}",a);
                        #[cfg(not(debug_assertions))]
                        None
                    },
                }
            }
            _ => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct GotoLayer {
    option: Option<LayerOption>
}
#[derive(Debug,Clone)]
pub enum LayerDirection {
    Forward,
    Backward,
    Value(Value),
}
impl LayerDirection {
    fn from(val: Option<Value>) -> Option<LayerDirection> {
        match val {
            Some(Value::String(a)) => {
                match a.as_str() {
                    "forward" => Some(Self::Forward),
                    "backward" => Some(Self::Backward),
                    _ => Some(Self::Value(Value::String(a))),
                }
            }
            _ => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ChangeLayer {
    direction: Option<LayerDirection>,
    by: Option<Value>,
}
#[derive(Debug,Clone)]
pub enum Costume {
    ByNumber(CostumeByNumber),
    ByName(CostumeByName),
    WithName(Option<Value>),
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct CostumeByNumber {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct CostumeByName {}

#[derive(Debug,Clone)]
pub enum Backdrop {
    ByNumber(BackdropByNumber),
    ByName(BackdropByName),
    WithName(Option<Value>),
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct BackdropByNumber {}
#[block_derive]
#[derive(Debug,Clone)]
pub struct BackdropByName {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct Size {}


//
// Sound blocks
//
#[block_derive]
#[derive(Debug,Clone)]
pub struct PlaySound {
    sound: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct PlaySoundUntilDone {
    sound: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct StartSound {
    sound: String,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct StopAllSounds {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct SoundEffectsMenu {
    option: Option<SoundEffect>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SoundSoundsMenu {
    option: Option<Value>
}
#[derive(Debug,Clone)]
pub enum SoundEffect {
    Pitch,
    Pan
}
impl SoundEffect {
    fn from(val: Option<Value>) -> Option<SoundEffect> {
        match val {
            Some(Value::String(a)) => {
                match a.as_str() {
                    "PITCH" => Some(Self::Pitch),
                    "PAN" => Some(Self::Pan),
                    _ => {
                        #[cfg(debug_assertions)]
                        panic!("invalid sound effect: {}",a);
                        #[cfg(not(debug_assertions))]
                        None
                    },
                }
            }
            _ => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ChangeEffectBy {
    effect: Option<Value>,
    units: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SetEffectTo {
    effect: Option<Value>,
    percentage: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ClearSoundEffects {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct ChangeVolumeBy {
    units: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SetVolumeTo {
    percentage: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Volume {}


// Event blocks
#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenGreenFlagClicked {}

#[derive(Debug,Clone)]
pub enum Key {
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    Space,
    Any,
    /// Any character from a-z and 0-9
    Alphanumerical(char),
}
impl Key {
    fn from(val: Option<Value>) -> Option<Key> {
        match val {
            Some(Value::String(a)) => {
                match a.as_str() {
                    "up arrow" => Some(Self::UpArrow),
                    "down arrow" => Some(Self::DownArrow),
                    "right arrow" => Some(Self::RightArrow),
                    "left arrow" => Some(Self::LeftArrow),
                    "space" => Some(Self::Space),
                    "any" => Some(Self::Any),
                    _ => {
                        Some(Self::Alphanumerical(a.chars().next().unwrap()))
                    }
                }
            }
            _ => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenKeyPressed {
    key: Option<Key>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenSpriteClicked {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenStageClicked {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenBackdropSwitchesTo {
    backdrop: Option<Value>,
}
#[derive(Debug,Clone)]
pub enum EventOption {
    Loudness,
    Timer
}
impl EventOption {
    fn from(val: Option<Value>) -> Option<EventOption> {
        match val {
            Some(Value::String(a)) => {
                match a.as_str() {
                    "loudness" => Some(Self::Loudness),
                    "timer" => Some(Self::Timer),
                    _ => {
                        #[cfg(debug_assertions)]
                        panic!("invalid event option: {}",a);
                        #[cfg(not(debug_assertions))]
                        None
                    },
                }
            }
            _ => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenOptionGreaterThen {
    option: Option<EventOption>,
    by: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenIRecieveBroadcast {
    broadcast: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Broadcast {
    broadcast: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct BroadcastAndWait {
    broadcast: Option<Value>,
}

//
// Control
//
#[block_derive]
#[derive(Debug,Clone)]
pub struct WaitSeconds {
    seconds: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Repeat {
    units: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Forever {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct IfThen {
    condition: Option<Value>,
    then: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct IfThenElse {
    condition: Option<Value>,
    then: Option<Value>,
    otherwise: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct WaitUntil {
    condition: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct RepeatUntil {
    condition: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct StopAll {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct WhenIStartAsAClone {}

#[derive(Debug,Clone)]
pub enum SpriteOption {
    Myself,
    Sprite(String),
}
impl SpriteOption {
    fn from(val: Option<Value>) -> Option<SpriteOption> {
        let myself: &'static str = "_myself_";
        match val {
            Some(Value::String(a)) => {
                match a.as_str() {
                    myself => Some(Self::Myself),
                    _ => Some(Self::Sprite(a)),
                }
            }
            _ => {
                #[cfg(debug_assertions)]
                panic!("invalid sprite option: {:?}",val);
                #[cfg(not(debug_assertions))]
                None
            },
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct CreateCloneOf {
    of: Option<SpriteOption>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct DeleteClone {}


// Sensing blocks
#[derive(Debug,Clone)]
pub enum SensingOption {
    MousePointer,
    Edge,
    Sprite(Value)
}
impl SensingOption {
    fn from(val: Option<Value>) -> Option<SensingOption> {
        match val {
            Some(Value::String(a)) => {
                match a.as_str() {
                    "_mouse_" => Some(Self::MousePointer),
                    "_edge_" => Some(Self::Edge),
                    _ => Some(Self::Sprite(Value::String(a))),
                }
            }
            _ => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Touching {
    touching: Option<Value>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct TouchingMenu {
    touching: Option<SensingOption>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct TouchingColor {
    color: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct ColorTouchingColor {
    color1: Option<Value>,
    color2: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct DistanceTo {
    to: Option<SensingOption>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Answer {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct KeyPressed {
    key: Option<Key>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct MouseDown {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct MouseX {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct MouseY {}

#[derive(Debug,Clone)]
pub enum DraggableOption {
    Draggable,
    NotDraggable
}
impl DraggableOption {
    fn from(val: Option<Value>) -> Option<DraggableOption> {
        let drag: &'static str = "draggable";
        let not_drag: &'static str = "not draggable";
        match val {
            Some(a) => {
                match a {
                    drag => Some(Self::Draggable),
                    not_drag => Some(Self::NotDraggable),
                    _ => {
                        #[cfg(debug_assertions)]
                        panic!("invalid draggable option: {:?}",a);
                        #[cfg(not(debug_assertions))]
                        None
                    },
                }
            }
            None => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct SetDragMode {
    option: Option<DraggableOption>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Loudness {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct Timer {}

#[block_derive]
#[derive(Debug,Clone)]
pub struct ResetTimer {}

#[derive(Debug,Clone)]
pub enum BackdropOfOption {
    BackdropNumber,
    BackdropName,
    Volume,
    MyVariable,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct BackdropOf {
    backdrop: BackdropOfOption,
}
#[derive(Debug,Clone)]
pub enum CurrentTimeOption {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second,
}
impl CurrentTimeOption {
    fn from(val: Option<Value>) -> Option<CurrentTimeOption> {
        match val {
            Some(a) => {
                match a {
                    Value::String(a) => match a.as_str() {
                        "year" => Some(Self::Year),
                        "month" => Some(Self::Month),
                        "date" => Some(Self::Date),
                        "day of week" => Some(Self::DayOfWeek),
                        "hour" => Some(Self::Hour),
                        "minute" => Some(Self::Minute),
                        "second" => Some(Self::Second),
                        _ => {
                            #[cfg(debug_assertions)]
                            panic!("invalid time option: {}",a);
                            #[cfg(not(debug_assertions))]
                            None
                        },
                    }
                    _ => {
                        #[cfg(debug_assertions)]
                        panic!("invalid type given for current time, expected string");
                        #[cfg(not(debug_assertions))]
                        None
                    },
                }
            }
            None => None,
        }
    }
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct CurrentTime {
    option: Option<CurrentTimeOption>
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct DaysSince2000;#[block_derive]
#[derive(Debug,Clone)]
pub struct Username {}


//
// Operators
//
#[block_derive]
#[derive(Debug,Clone)]
pub struct Add {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Sub {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Mul {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Divide {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct PickRandom {
    min: Option<Value>,
    max: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct GreaterThen {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct LesserThen {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct EqualTo {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct And {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Or {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Not {
    a: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Join {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct LetterOf {
    index: Option<Value>,
    a: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct LengthOf {
    a: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Contains {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Modulo {
    a: Option<Value>,
    b: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Round {
    a: Option<Value>,
}
#[block_derive]
#[derive(Debug,Clone)]
pub struct Absolute {
    a: Option<Value>,
}

#[block_derive]
#[derive(Debug,Clone)]
pub struct UnusedOpcode {
    name: String,
}

#[block_derive]
#[derive(Debug,Clone)]
pub struct InvalidOpcode {
    name: String,
}

// Variables

// todo

// Deserializiation implementation

// numbers only regex
lazy_static! {
    static ref NUMBERS_ONLY_REGEX: Regex = Regex::new(r"[^0-9]").unwrap();
}

impl<'de> Deserialize<'de> for BlockType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>{
        let v: HashMap<String, SerdeValue> = HashMap::deserialize(d)?;

        match v.get_key_value("opcode") {
            Some(a) => {
                let name: &String = match a.1 {
                    SerdeValue::String(a) => a,
                    _ => {
                        return Err(format!("not a string: {}",a.1)).map_err(de::Error::custom);
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
                let (_, inputs) = v.get_key_value("inputs").unwrap();
                let inputs = inputs.as_object().unwrap();

                // same with fields
                let (_, fields) = v.get_key_value("fields").unwrap();
                let fields = fields.as_object().unwrap();

                //
                // get the relevant values.
                let mut values: Vec<Value> = Vec::new();
                for (_, input) in inputs {
                    let b = input.get(1);
                    let block = match b {
                        Some(a) => {
                            let bl;
                            if a.is_array() {
                                bl = a.as_array().unwrap().get(1).unwrap();
                            } else {
                                bl = a
                            };
                            if bl.is_null() {
                                continue
                            } else if bl.is_u64() {
                                Value::Number(bl.as_u64().unwrap() as f64)
                            } else if bl.is_i64() {
                                Value::Number(bl.as_i64().unwrap() as f64)
                            } else if bl.is_f64() {
                                Value::Number(bl.as_f64().unwrap())
                            } else {
                                let st = bl.as_str().unwrap();
                                if NUMBERS_ONLY_REGEX.is_match(st) {
                                    Value::String(st.to_string())
                                } else {
                                    match st.parse() {
                                        Ok(a) => Value::Number(a),
                                        Err(_) => return Err(format!("could not format {} into a number",st)).map_err(de::Error::custom),
                                    }

                                }
                            }
                        },
                        None => {return Err(format!("what")).map_err(de::Error::custom);}// we should never reach this
                    };
                    values.push(block);
                }
                let mut field_values: Vec<Value> = Vec::new();
                for (_, field) in fields {
                    let b = field.get(0);
                    let value = match b {
                        Some(a) => {
                            let bl;
                            if a.is_array() {
                                bl = a.as_array().unwrap().get(1).unwrap();
                            } else {
                                bl = a
                            };
                            if bl.is_null() {
                                continue
                            } else if bl.is_u64() {
                                Value::Number(bl.as_u64().unwrap() as f64)
                            } else if bl.is_i64() {
                                Value::Number(bl.as_i64().unwrap() as f64)
                            } else if bl.is_f64() {
                                Value::Number(bl.as_f64().unwrap())
                            } else {
                                let st = bl.as_str().unwrap();
                                if NUMBERS_ONLY_REGEX.is_match(st) {
                                    Value::String(st.to_string())
                                } else {
                                    match st.parse() {
                                        Ok(a) => Value::Number(a),
                                        Err(_) => return Err(format!("could not format {} into a number",st)).map_err(de::Error::custom),
                                    }

                                }
                            }
                        },
                        None => {return Err(format!("what")).map_err(de::Error::custom);}// we should never reach this
                    };
                    field_values.push(value);
                }

                let val1 = match values.get(0) {
                    Some(a) => Some(a.to_owned()),
                    None => None,
                };
                let val2 = match values.get(1) {
                    Some(a) => Some(a.to_owned()),
                    None => None,
                };
                let val3 = match values.get(2) {
                    Some(a) => Some(a.to_owned()),
                    None => None,
                };

                let field1 = match field_values.get(0) {
                    Some(a) => Some(a.to_owned()),
                    None => None,
                };
                let field2 = match field_values.get(1) {
                    Some(a) => Some(a.to_owned()),
                    None => None,
                };
                let field3 = match field_values.get(2) {
                    Some(a) => Some(a.to_owned()),
                    None => None,
                };

                let prev = match v.get_key_value("parent") {
                    Some(a) => Some(a.to_owned().1.to_string()),
                    None => None,
                };
                let next = match v.get_key_value("next") {
                    Some(a) => Some(a.to_owned().1.to_string()),
                    None => None,
                };

                match &*name.to_string() {
                    block_names::MOTION_MOVE => {
                        Ok(BlockType::Move(Move { steps: val1, prev, next }))
                    },
                    block_names::MOTION_GOTO_XY => {
                        Ok(BlockType::Goto(Goto::Pos(
                            GotoPos {
                                x: val1,
                                y: val2,
                                prev, next
                            }
                        )))
                    },
                    // redundant, just goes to MOTION_GOTO_MENU
                    block_names::MOTION_GOTO => {
                        Ok(BlockType::UnusedOpcode(UnusedOpcode{
                            name: block_names::MOTION_GOTO.to_string(),
                            prev, next
                        }))
                    },
                    block_names::MOTION_GOTO_MENU => {
                        Ok(BlockType::Goto(Goto::Option(
                            GotoOption {
                                option: MovementOption::from(val1),
                                prev, next
                            }
                        )))
                    },
                    block_names::MOTION_TURN_LEFT => {
                        Ok(BlockType::RotateLeft(RotateLeft{
                            degrees: val1,
                            prev, next
                        }))
                    },
                    block_names::MOTION_TURN_RIGHT => {
                        Ok(BlockType::RotateRight(RotateRight{
                            degrees: val1,
                            prev, next
                        }))
                    },
                    // Unused: Mouse. It's always mouse.
                    block_names::MOTION_POINT_MENU => {
                        Ok(BlockType::Point(Point::Towards(
                            PointOption { option: Some(Value::String(String::from("_mouse_"))), prev, next }
                        )))
                    }

                    block_names::MOTION_POINT_DIRECTION => {
                        Ok(BlockType::Point(Point::Direction(
                            PointDirection { x: val1, y: val2, prev, next }
                        )))
                    },
                    block_names::MOTION_POINT_TOWARDS => {
                        Ok(BlockType::Point(Point::Towards(
                            PointOption { option: val1, prev, next }
                        )))
                    },
                    block_names::MOTION_GLIDE_SECONDS_TO_XY => {
                        Ok(BlockType::Glide(Glide::Pos(
                            GlidePos{ x: val1, y: val2, prev, next }
                        )))
                    },
                    // redundant, just goes to MOTION_GLIDE_TO_MENU
                    block_names::MOTION_GLIDE_TO => {
                        Ok(BlockType::UnusedOpcode(UnusedOpcode {
                            name: block_names::MOTION_GLIDE_TO.to_string(),
                            prev, next
                        }))
                    },

                    block_names::MOTION_GLIDE_TO_MENU => {
                        Ok(BlockType::Glide(Glide::Option(
                            GlideOption { option: MovementOption::from(field1), prev, next}
                        )))
                    }

                    block_names::MOTION_IF_ON_EDGE_BOUNCE => {
                        Ok(BlockType::IfOnEdgeBounce(IfOnEdgeBounce{prev, next}))
                    },
                    block_names::MOTION_SET_ROTATION_STYLE => {
                        Ok(BlockType::SetRotationStyle(SetRotationStyle{
                            style: RotationStyle::from(val1),
                            prev, next
                        }))
                    },
                    block_names::MOTION_CHANGE_X_BY => {
                        Ok(BlockType::ChangeX(
                            ChangeX{
                                x: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::MOTION_SET_X => {
                        Ok(BlockType::SetX(
                            SetX { x: val1, prev, next }
                        ))
                    }
                    block_names::MOTION_CHANGE_Y_BY => {
                        Ok(BlockType::ChangeY(
                            ChangeY { y: val1, prev, next }
                        ))
                    }
                    block_names::MOTION_SET_Y => {
                        Ok(BlockType::SetY(
                            SetY {y: val1, prev, next}
                        ))
                    }
                    block_names::MOTION_XPOSITION => {
                        Ok(BlockType::XPosition(
                            XPosition{prev, next}
                        ))
                    }
                    block_names::MOTION_YPOSITION => {
                        Ok(BlockType::YPosition(
                            YPosition{prev, next}
                        ))
                    }
                    block_names::MOTION_DIRECTION => {
                        Ok(BlockType::Direction(
                            Direction{prev, next}
                        ))
                    }
                    // presumed unused
                    block_names::MOTION_SCROLL_RIGHT => {
                        todo!()
                    }
                    block_names::MOTION_SCROLL_UP => {
                        todo!()
                    }
                    block_names::MOTION_ALIGN_SCENE => {
                        todo!()
                    }
                    block_names::MOTION_XSCROLL => {
                        todo!()
                    }
                    block_names::MOTION_YSCROLL => {
                        todo!()
                    }

                    block_names::LOOKS_SAY => {
                        Ok(BlockType::SayForever(
                            SayForever{
                                message: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_SAY_FOR_SECS => {
                        Ok(BlockType::Say(
                            Say{
                                message: val1,
                                secs: val2,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_THINK => {
                        Ok(BlockType::ThinkForever(
                            ThinkForever {
                                message: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_THINK_FOR_SECS => {
                        Ok(BlockType::Think(
                            Think {
                                message: val1,
                                secs: val2,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_SHOW => {
                        Ok(BlockType::ShowSprite(ShowSprite{prev, next}))
                    }
                    block_names::LOOKS_HIDE => {
                        Ok(BlockType::HideSprite(HideSprite{prev, next}))
                    }
                    block_names::LOOKS_HIDE_ALL_SPRITES => {
                        Ok(BlockType::HideAllSprites(HideAllSprites{prev, next}))
                    }
                    block_names::LOOKS_SWITCH_COSTUME_TO => {
                        Ok(BlockType::SwitchCostume(
                            SwitchCostume {
                                costume: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_SWITCH_BACKDROP_TO => {
                        Ok(BlockType::SwitchBackdrop(
                            SwitchBackdrop {
                                backdrop: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_SWITCH_BACKDROP_TO_AND_WAIT => {
                        Ok(BlockType::SwitchBackdropAndWait(
                            SwitchBackdropAndWait {
                                backdrop: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_NEXT_COSTUME => {
                        Ok(BlockType::NextCostume(
                            NextCostume{prev, next}
                        ))
                    }
                    block_names::LOOKS_NEXT_BACKDROP => {
                        Ok(BlockType::NextBackdrop(
                            NextBackdrop{prev, next}
                        ))
                    }
                    block_names::LOOKS_CHANGE_EFFECT_BY => {
                        Ok(BlockType::ChangeEffectBy(
                            ChangeEffectBy {
                                effect: val1,
                                units: val2,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_SET_EFFECT_TO => {
                        Ok(BlockType::SetEffectTo(
                            SetEffectTo {
                                effect: val1,
                                percentage: val2,
                                prev, next
                            }
                        ))
                    }
                    block_names::LOOKS_CLEAR_GRAPHICS_EFFECTS => {
                        Ok(BlockType::ClearGraphicEffects(
                            ClearGraphicEffects{prev, next}
                        ))
                    }
                    block_names::LOOKS_CHANGE_SIZE_BY => {
                        Ok(BlockType::ChangeSize(ChangeSize { units: val1, prev, next }))
                    }
                    block_names::LOOKS_SET_SIZE_TO => {
                        Ok(BlockType::SetSize(SetSize { percentage: val1, prev, next }))
                    }
                    block_names::LOOKS_CHANGE_STRETCH_BY => {
                        todo!()
                    }
                    block_names::LOOKS_SET_STRETCH_TO => {
                        todo!()
                    }
                    block_names::LOOKS_GOTO_FRONT_BACK => {
                        Ok(BlockType::GotoLayer(GotoLayer{
                            option: LayerOption::from(val1),
                            prev, next
                        }))
                    }
                    block_names::LOOKS_GO_FORWARD_BACKWARD_LAYERS => {
                        Ok(BlockType::ChangeLayer(ChangeLayer{
                            direction: LayerDirection::from(val1),
                            by: val2,
                            prev, next
                        }))
                    }
                    block_names::LOOKS_SIZE => {
                        Ok(BlockType::Size(Size{prev, next}))
                    }
                    block_names::LOOKS_COSTUME => {
                        Ok(BlockType::Costume(
                            Costume::WithName(field1)
                        ))
                    }
                    block_names::LOOKS_COSTUME_NUMBER_NAME => {
                        match field1 {
                            Some(Value::String(a)) => match a.as_str() {
                                "number" => Ok(BlockType::Costume(
                                    Costume::ByNumber(CostumeByNumber{prev, next})
                                )),
                                "name" => Ok(BlockType::Costume(
                                    Costume::ByName(CostumeByName{prev, next})
                                )),
                                _ => {return Err(format!("invalid option given for costume number/name")).map_err(de::Error::custom);},
                            }
                            _ => {return Err(format!("no option given for costume number/name")).map_err(de::Error::custom);},
                        }
                    }
                    block_names::LOOKS_BACKDROP => {
                        Ok(BlockType::Backdrop(
                            Backdrop::WithName(field1)
                        ))
                    }
                    block_names::LOOKS_BACKDROP_NUMBER_NAME => {
                        match field1 {
                            Some(Value::String(a)) => match a.as_str() {
                                "number" => Ok(BlockType::Backdrop(
                                    Backdrop::ByNumber(BackdropByNumber{prev, next})
                                )),
                                "name" => Ok(BlockType::Backdrop(
                                    Backdrop::ByName(BackdropByName{prev, next})
                                )),
                                _ => {return Err(format!("invalid option given for backdrop number/name")).map_err(de::Error::custom);},
                            }
                            _ => {return Err(format!("no option given for backdrop number/name")).map_err(de::Error::custom);},
                        }
                    }
                    block_names::SOUND_PLAY => {
                        Ok(BlockType::PlaySound(PlaySound{
                            sound: val1,
                            prev, next
                        }))
                    }
                    block_names::SOUND_PLAY_UNTIL_DONE => {
                        Ok(BlockType::PlaySoundUntilDone(PlaySoundUntilDone{
                            sound: val1,
                            prev, next
                        }))
                    }
                    block_names::SOUND_STOP_ALL_SOUNDS => {
                        Ok(BlockType::StopAllSounds(StopAllSounds{prev, next}))
                    }
                    block_names::SOUND_SET_EFFECT_TO => {
                        Ok(BlockType::SetEffectTo(SetEffectTo{
                            effect: val1,
                            percentage: val2,
                            prev, next
                        }))
                    }
                    block_names::SOUND_CHANGE_EFFECT_BY => {
                        Ok(BlockType::ChangeEffectBy(ChangeEffectBy{
                            effect: val1,
                            units: val2,
                            prev, next
                        }))
                    }
                    block_names::SOUND_CLEAR_EFFECTS => {
                        Ok(BlockType::ClearSoundEffects(ClearSoundEffects{prev, next}))
                    }
                    block_names::SOUND_SET_VOLUME_TO => {
                        Ok(BlockType::SetVolumeTo(SetVolumeTo{
                            percentage: val1,
                            prev, next
                        }))
                    }
                    block_names::SOUND_CHANGE_VOLUME_BY => {
                        Ok(BlockType::ChangeVolumeBy(ChangeVolumeBy{
                            units: val1,
                            prev, next
                        }))
                    }
                    block_names::SOUND_VOLUME => {
                        Ok(BlockType::Volume(Volume{prev, next}))
                    }

                    block_names::EVENT_WHEN_TOUCHING_OBJECT => {
                        todo!()
                    }
                    block_names::EVENT_BROADCAST => {
                        Ok(BlockType::Broadcast(Broadcast { broadcast: val1, prev, next }))
                    }
                    block_names::EVENT_BROADCAST_AND_WAIT => {
                        Ok(BlockType::BroadcastAndWait(BroadcastAndWait { broadcast: val1, prev, next }))
                    }
                    block_names::EVENT_WHEN_GREATER_THAN => {
                        Ok(BlockType::WhenOptionGreaterThen(
                            WhenOptionGreaterThen {
                                option: EventOption::from(val1),
                                by: val2,
                                prev, next
                            }
                        ))
                    }
                    block_names::EVENT_WHEN_FLAG_CLICKED => {
                        Ok(BlockType::WhenGreenFlagClicked(WhenGreenFlagClicked{prev, next}))
                    }
                    block_names::EVENT_WHEN_KEY_PRESSED => {
                        Ok(BlockType::WhenKeyPressed(WhenKeyPressed{
                            key: Key::from(val1),
                            prev, next
                        }))
                    }
                    block_names::EVENT_WHEN_THIS_SPRITECLICKED => {
                        Ok(BlockType::WhenSpriteClicked(WhenSpriteClicked{prev, next}))
                    }
                    block_names::EVENT_WHEN_STAGE_CLICKED => {
                        Ok(BlockType::WhenStageClicked(WhenStageClicked{prev, next}))
                    }
                    block_names::EVENT_WHEN_BACKDROP_SWITCHESTO => {
                        Ok(BlockType::WhenBackdropSwitchesTo(
                            WhenBackdropSwitchesTo{
                                backdrop: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::EVENT_WHEN_BROADCAST_RECEIVED => {
                        Ok(BlockType::WhenIRecieveBroadcast(
                            WhenIRecieveBroadcast{
                                broadcast: val1,
                                prev, next
                            }
                        ))
                    }
                    
                    block_names::CONTROL_REPEAT => {
                        Ok(BlockType::Repeat(
                            Repeat{
                                units: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::CONTROL_REPEAT_UNTIL => {
                        Ok(BlockType::RepeatUntil(
                            RepeatUntil{
                                condition: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::CONTROL_WHILE => {
                        Ok(BlockType::RepeatUntil(
                            RepeatUntil{
                                condition: val1,
                                prev, next
                            }
                        ))
                    }
                    block_names::CONTROL_FOR_EACH => {
                        todo!() // i don't see this in the scratch part picker what?
                    }
                    block_names::CONTROL_FOREVER => {
                        Ok(BlockType::Forever(
                            Forever{prev, next}
                        ))
                    }
                    block_names::CONTROL_WAIT => {
                        Ok(BlockType::WaitSeconds(WaitSeconds{
                            seconds: val1,
                            prev, next
                        }))
                    }
                    block_names::CONTROL_WAIT_UNTIL => {
                        Ok(BlockType::WaitUntil(WaitUntil{
                            condition: val1,
                            prev, next
                        }))
                    }
                    block_names::CONTROL_IF => {
                        Ok(BlockType::IfThen(
                            IfThen{
                                condition: val1,
                                then: val2,
                                prev, next
                            }
                        ))
                    }
                    block_names::CONTROL_IF_ELSE => {
                        Ok(BlockType::IfThenElse(
                            IfThenElse{
                                condition: val1,
                                then: val2,
                                otherwise: val3,
                                prev, next
                            }
                        ))
                    }
                    block_names::CONTROL_STOP => {
                        Ok(BlockType::StopAll(StopAll{prev, next}))
                    }
                    block_names::CONTROL_CREATE_CLONE_OF => {
                        Ok(BlockType::CreateCloneOf(CreateCloneOf{
                            of: SpriteOption::from(val1),
                            prev, next
                        }))
                    }
                    block_names::CONTROL_DELETE_THIS_CLONE => {
                        Ok(BlockType::DeleteClone(DeleteClone{prev, next}))
                    }
                    block_names::CONTROL_GET_COUNTER => {
                        todo!()
                    }
                    block_names::CONTROL_INCREMENT_COUNTER => {
                        todo!()
                    }
                    block_names::CONTROL_CLEAR_COUNTER => {
                        todo!()
                    }
                    block_names::CONTROL_ALL_AT_ONCE => {
                        todo!()
                    }
                    block_names::CONTROL_START_AS_CLONE => {
                        todo!()
                    }

                    block_names::SENSING_TOUCHING_OBJECT_MENU => {
                        Ok(BlockType::TouchingMenu(TouchingMenu{
                            touching: SensingOption::from(field1),
                            prev, next
                        }))
                    }

                    block_names::SENSING_TOUCHING_OBJECT => {
                        Ok(BlockType::Touching(Touching{
                            touching: val1,
                            prev, next
                        }))
                    }

                    block_names::SENSING_TOUCHING_COLOR => {
                        Ok(BlockType::TouchingColor(TouchingColor {
                            color: val1,
                            prev, next
                        }))
                    }
                    block_names::SENSING_COLOR_IS_TOUCHING_COLOR => {
                        Ok(BlockType::ColorTouchingColor(ColorTouchingColor {
                            color1: val1,
                            color2: val2,
                            prev, next
                        }))
                    }
                    block_names::SENSING_DISTANCE_TO => {
                        Ok(BlockType::DistanceTo(DistanceTo{
                            to: SensingOption::from(val1),
                            prev, next
                        }))
                    }
                    block_names::SENSING_TIMER => {
                        Ok(BlockType::Timer(Timer{prev, next}))
                    }
                    block_names::SENSING_RESET_TIMER => {
                        Ok(BlockType::ResetTimer(ResetTimer{prev, next}))
                    }
                    // uses fields.
                    block_names::SENSING_OF => {
                        todo!()
                    }
                    block_names::SENSING_MOUSE_X => {
                        Ok(BlockType::MouseX(MouseX{prev, next}))
                    }
                    block_names::SENSING_MOUSE_Y => {
                        Ok(BlockType::MouseY(MouseY{prev, next}))
                    }
                    block_names::SENSING_SET_DRAG_MODE => {
                        Ok(BlockType::SetDragMode(SetDragMode{
                            option: DraggableOption::from(val1),
                            prev, next
                        }))
                    }
                    block_names::SENSING_MOUSE_DOWN => {
                        Ok(BlockType::MouseDown(MouseDown{prev, next}))
                    }
                    block_names::SENSING_KEY_PRESSED => {
                        Ok(BlockType::KeyPressed(KeyPressed{
                            key: Key::from(val1),
                            prev, next
                        }))
                    }
                    block_names::SENSING_CURRENT => {
                        Ok(BlockType::CurrentTime(CurrentTime{
                            option: CurrentTimeOption::from(val1),
                            prev, next
                        }))
                    }
                    block_names::SENSING_DAYS_SINCE_2000 => {
                        Ok(BlockType::DaysSince2000(DaysSince2000{}))
                    }
                    block_names::SENSING_LOUDNESS => {
                        Ok(BlockType::Loudness(Loudness{prev, next}))
                    }
                    block_names::SENSING_LOUD => {
                        todo!() // What?
                    }
                    block_names::SENSING_ASK_AND_WAIT => {
                        todo!()
                    }
                    block_names::SENSING_ANSWER => {
                        Ok(BlockType::Answer(Answer{prev, next}))
                    }
                    block_names::SENSING_USERNAME => {
                        Ok(BlockType::Username(Username{prev, next}))
                    }
                    block_names::SENSING_USER_ID => {
                        todo!()
                    }
                    
                    block_names::OPERATOR_ADD => {
                        Ok(BlockType::Add(Add{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_SUBTRACT => {
                        Ok(BlockType::Sub(Sub{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_MULTIPLY => {
                        Ok(BlockType::Mul(Mul{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_DIVIDE => {
                        Ok(BlockType::Divide(Divide{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_LESSER_THEN => {
                        Ok(BlockType::LesserThen(LesserThen{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_EQUALS => {
                        Ok(BlockType::EqualTo(EqualTo{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_GREATER_THEN => {
                        Ok(BlockType::GreaterThen(GreaterThen{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_AND => {
                        Ok(BlockType::And(And{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_OR => {
                        Ok(BlockType::Or(Or{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_NOT => {
                        Ok(BlockType::Not(Not{
                            a: val1,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_RANDOM => {
                        Ok(BlockType::PickRandom(PickRandom{
                            min: val1,
                            max: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_JOIN => {
                        Ok(BlockType::Join(Join{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_LETTER_OF => {
                        Ok(BlockType::LetterOf(LetterOf{
                            index: val1,
                            a: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_LENGTH => {
                        Ok(BlockType::LengthOf(LengthOf{
                            a: val1,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_CONTAINS => {
                        Ok(BlockType::Contains(Contains{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_MOD => {
                        Ok(BlockType::Modulo(Modulo{
                            a: val1,
                            b: val2,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_ROUND => {
                        Ok(BlockType::Round(Round{
                            a: val1,
                            prev, next
                        }))
                    }
                    block_names::OPERATOR_MATHOP => {
                        Ok(BlockType::Absolute(Absolute{
                            a: val1,
                            prev, next
                        }))
                    }

                    block_names::SOUND_SOUNDS_MENU => {
                        Ok(BlockType::SoundSoundsMenu(SoundSoundsMenu{
                            option: val1,
                            prev, next
                        }))
                    }

                    // Unused
                    /*block_names::SOUNDS_BEATS_MENU => {

                    }*/

                    block_names::SOUND_EFFECTS_MENU => {
                        Ok(BlockType::SoundEffectsMenu(SoundEffectsMenu{
                            option: SoundEffect::from(val1),
                            prev, next
                        }))
                    }



                    _ => {
                        #[cfg(debug_assertions)]
                        return Err(format!("invalid opcode {}",a.1)).map_err(de::Error::custom);
                        #[cfg(not(debug_assertions))]
                        Ok(BlockType::InvalidOpcode(InvalidOpcode{
                            name: name.to_string(),
                        }))
                    }

                }
            },
            None => {
                return Err("no opcode").map_err(de::Error::custom);
            },
        }
    }
}