use std::collections::HashMap;

#[allow(dead_code)]

use serde::{Deserialize, Deserializer ,de};
use serde_json::Value as SerdeValue;

/// Enums and structs that represent certain types of blocks in Scratch.
///
/// It should be noted that the following rules are applied when translating
/// these blocks:
/// - Blocks that can mimmicked by other blocks are combined. For example, "turn clockwise" and "turn counterclickwise" are combined into one "turn block", and the library will present the latter block with a negative number.
/// - Blocks that are the same but with a different argument are represented by an enum that leads to the respective types. For example, "goto <option>" and "goto "<x, y>" are one "goto" enum with a "Pos" and "Option" option.
/// - Since there's no logic here, Scratch's "sprite globals" i.e. "x position" are also represented by structs, and its up to you to resolve them. This also goes for functions with no arguments such as "next costume"
/// - Some blocks have special cases where things are combined. Those are documented as such.
///

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
    /// turn clockwise or counterclockwise _ degrees.
    /// counterclockwise is represented by a negative number
    Rotate(Rotate),
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
    Think(Think),
    SwitchCostume(SwitchCostume),
    NextCostume(NextCostume),
    SwitchBackdrop(SwitchBackdrop),
    NextBackdrop(NextBackdrop),
    ChangeSize(ChangeSize),
    SetSize(SetSize),
    ClearGraphicEffects(ClearGraphicEffects),
    ShowSprite(ShowSprite),
    HideSprite(HideSprite),
    GotoLayer(GotoLayer),
    ChangeLayer(ChangeLayer),
    Costume(Costume),
    Backdrop(Backdrop),
    Size(Size),

    // Sound blocks

    PlaySoundUntilDone(PlaySoundUntilDone),
    StartSound(StartSound),
    StopAllSounds(StopAllSounds),
    ChangeEffectBy(ChangeEffectBy),
    SetEffectTo(SetEffectTo),
    ChangeVolumeBy(ChangeVolumeBy),
    SetVolumeTo(ChangeVolumeBy),
    Volume(Volume),

    // Event blocks

    WhenGreenFlagClicked(WhenGreenFlagClicked),
    WhenKeyClicked(WhenKeyClicked),
    WhenSpriteClicked(WhenSpriteClicked),
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
    
    //
    // Operator blocks
    //
    
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

}

//
// Motion Blocks
//
#[derive(Debug,Clone)]
pub struct Move {
    steps: Value
}
#[derive(Debug,Clone)]
pub struct Rotate {
    degrees: Value
}
#[derive(Debug,Clone)]
pub enum MovementOption {
    RandomPosition,
    MousePointer,
}
#[derive(Debug,Clone)]
pub enum Goto {
    Pos(GotoPos),
    Option(GotoOption),
}
#[derive(Debug,Clone)]
pub struct GotoPos {
    x: Option<String>,
    y: Option<String>,
}
#[derive(Debug,Clone)]
pub struct GotoOption {
    option: MovementOption,
}
#[derive(Debug,Clone)]
pub enum Glide {
    Pos(GlidePos),
    Option(GlideOption),
}
#[derive(Debug,Clone)]
pub struct GlidePos {
    x: Option<Value>,
    y: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct GlideOption {
    option: MovementOption,
}
#[derive(Debug,Clone)]
pub enum Point {
    Direction(PointDirection),
    Towards(PointOption),
}
#[derive(Debug,Clone)]
pub struct PointDirection {
    x: Option<Value>,
    y: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct PointOption {
    option: MovementOption,
}
#[derive(Debug,Clone)]
pub struct ChangeX {
    x: Value
}
#[derive(Debug,Clone)]
pub struct SetX {
    x: Value
}
#[derive(Debug,Clone)]
pub struct ChangeY {
    y: Value
}
#[derive(Debug,Clone)]
pub struct SetY {
    y: Value
}
#[derive(Debug,Clone)]
pub struct IfOnEdgeBounce {}
#[derive(Debug,Clone)]
pub enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}
#[derive(Debug,Clone)]
pub struct SetRotationStyle {
    style: RotationStyle
}
#[derive(Debug,Clone)]
pub struct XPosition;#[derive(Debug,Clone)]
pub struct YPosition;#[derive(Debug,Clone)]
pub struct Direction;

//
// Look blocks
//
#[derive(Debug,Clone)]
pub struct SayForever {
    message: String,
}
#[derive(Debug,Clone)]
pub struct Say {
    message: String,
    secs: Option<String>
}
#[derive(Debug,Clone)]
pub struct Think {
    message: String,
    secs: Option<String>
}
#[derive(Debug,Clone)]
pub struct SwitchCostume {
    costume: Option<String>,
}
#[derive(Debug,Clone)]
pub struct NextCostume;
#[derive(Debug,Clone)]
pub struct SwitchBackdrop {
    backdrop: Option<String>,
}
#[derive(Debug,Clone)]
pub struct NextBackdrop;
#[derive(Debug,Clone)]
pub struct ChangeSize {
    units: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct SetSize {
    percentage: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct ClearGraphicEffects;#[derive(Debug,Clone)]
pub struct ShowSprite;#[derive(Debug,Clone)]
pub struct HideSprite;
#[derive(Debug,Clone)]
pub enum LayerOption {
    Front,
    Back,
}
#[derive(Debug,Clone)]
pub struct GotoLayer {
    option: LayerOption
}
#[derive(Debug,Clone)]
pub enum LayerDirection {
    Forward,
    Backward,
}
#[derive(Debug,Clone)]
pub struct ChangeLayer {
    direction: LayerDirection,
    by: i32,
}
#[derive(Debug,Clone)]
pub enum Costume {
    ByNumber(CostumeByNumber),
    ByName(CostumeByName)
}
#[derive(Debug,Clone)]
pub struct CostumeByNumber;#[derive(Debug,Clone)]
pub struct CostumeByName;
#[derive(Debug,Clone)]
pub enum Backdrop {
    ByNumber(BackdropByNumber),
    ByName(BackdropByName)
}
#[derive(Debug,Clone)]
pub struct BackdropByNumber;#[derive(Debug,Clone)]
pub struct BackdropByName;
#[derive(Debug,Clone)]
pub struct Size;

//
// Sound blocks
//
#[derive(Debug,Clone)]
pub struct PlaySoundUntilDone {
    sound: String
}
#[derive(Debug,Clone)]
pub struct StartSound {
    sound: String,
}
#[derive(Debug,Clone)]
pub struct StopAllSounds;
#[derive(Debug,Clone)]
pub enum SoundEffect {
    Pitch,
    Pan
}
#[derive(Debug,Clone)]
pub struct ChangeEffectBy {
    effect: SoundEffect,
    units: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct SetEffectTo {
    effect: SoundEffect,
    percentage: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct ChangeVolumeBy {
    units: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct SetVolumeTo {
    percentage: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct Volume;

// Event blocks
#[derive(Debug,Clone)]
pub struct WhenGreenFlagClicked;
#[derive(Debug,Clone)]
pub enum Key {
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    Any,
    /// Any character from a-z and 0-9
    Alphanumerical(char),
}
#[derive(Debug,Clone)]
pub struct WhenKeyClicked {
    key: Key,
}
#[derive(Debug,Clone)]
pub struct WhenSpriteClicked;#[derive(Debug,Clone)]
pub struct WhenBackdropSwitchesTo {
    backdrop: String,
}
#[derive(Debug,Clone)]
pub enum EventOption {
    Loudness,
    Timer
}
#[derive(Debug,Clone)]
pub struct WhenOptionGreaterThen {
    option: EventOption,
    by: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct WhenIRecieveBroadcast {
    broadcast: String,
}
#[derive(Debug,Clone)]
pub struct Broadcast {
    broadcast: String,
}
#[derive(Debug,Clone)]
pub struct BroadcastAndWait {
    broadcast: String,
}

//
// Control
//
#[derive(Debug,Clone)]
pub struct WaitSeconds {
    seconds: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct Repeat {
    units: Option<Value>,
}
#[derive(Debug,Clone)]
pub struct Forever;
#[derive(Debug,Clone)]
pub struct IfThen {
    condition: Option<String>,
    then: Option<String>,
}
#[derive(Debug,Clone)]
pub struct IfThenElse {
    condition: Option<String>,
    then: Option<String>,
    otherwise: Option<String>,
}
#[derive(Debug,Clone)]
pub struct WaitUntil {
    condition: Option<String>,
}
#[derive(Debug,Clone)]
pub struct RepeatUntil {
    condition: Option<String>,
}
#[derive(Debug,Clone)]
pub struct StopAll;
#[derive(Debug,Clone)]
pub struct WhenIStartAsAClone;
#[derive(Debug,Clone)]
pub enum SpriteOption {
    Myself,
    Sprite(String),
}
#[derive(Debug,Clone)]
pub struct CreateCloneOf {
    of: SpriteOption,
}
#[derive(Debug,Clone)]
pub struct DeleteClone;

// Sensing blocks
#[derive(Debug,Clone)]
pub enum SensingOption {
    MousePointer,
    Edge,
    Sprite(String)
}
#[derive(Debug,Clone)]
pub struct Touching {
    touching: SensingOption
}
#[derive(Debug,Clone)]
pub struct TouchingColor {
    color: String,
}
#[derive(Debug,Clone)]
pub struct ColorTouchingColor {
    color1: String,
    color2: String,
}
#[derive(Debug,Clone)]
pub struct DistanceTo {
    to: SensingOption
}
#[derive(Debug,Clone)]
pub struct Answer;
#[derive(Debug,Clone)]
pub struct KeyPressed {
    key: Key
}
#[derive(Debug,Clone)]
pub struct MouseDown;#[derive(Debug,Clone)]
pub struct MouseX;#[derive(Debug,Clone)]
pub struct MouseY;
#[derive(Debug,Clone)]
pub enum DraggableOption {
    Draggable,
    NotDraggable
}
#[derive(Debug,Clone)]
pub struct SetDragMode {
    option: DraggableOption,
}
#[derive(Debug,Clone)]
pub struct Loudness;#[derive(Debug,Clone)]
pub struct Timer;
#[derive(Debug,Clone)]
pub struct ResetTimer;
#[derive(Debug,Clone)]
pub enum BackdropOfOption {
    BackdropNumber,
    BackdropName,
    Volume,
    MyVariable,
}
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
#[derive(Debug,Clone)]
pub struct CurrentTime {
    option: CurrentTimeOption
}
#[derive(Debug,Clone)]
pub struct DaysSince2000;#[derive(Debug,Clone)]
pub struct Username;

//
// Operators
//
#[derive(Debug,Clone)]
pub struct Add {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct Sub {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct Mul {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct Divide {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct PickRandom {
    min: Value,
    max: Value,
}
#[derive(Debug,Clone)]
pub struct GreaterThen {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct LesserThen {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct EqualTo {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct And {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct Or {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct Not {
    a: Value,
}
#[derive(Debug,Clone)]
pub struct Join {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct LetterOf {
    index: Value,
    a: Value,
}
#[derive(Debug,Clone)]
pub struct LengthOf {
    a: Value,
}
#[derive(Debug,Clone)]
pub struct Contains {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct Modulo {
    a: Value,
    b: Value,
}
#[derive(Debug,Clone)]
pub struct Round {
    a: Value,
}
#[derive(Debug,Clone)]
pub struct Absolute {
    a: Value,
}

// Variables

// todo

// Deserializiation implementation

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
                match &*name.to_string() {
                    "event_whenflagclicked" => {
                        Ok(BlockType::WhenGreenFlagClicked(WhenGreenFlagClicked{}))
                    }
                    _ => {
                        return Err(format!("invalid opcode {}",a.1)).map_err(de::Error::custom);
                    }
                }
            },
            None => {
                return Err("no opcode").map_err(de::Error::custom);
            },
        }
    }
}