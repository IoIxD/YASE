#[allow(dead_code)]

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
enum Value {
    Number(f64),
    String(String),
}

enum BlockType {
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

struct Move {
    steps: Value
}

struct Rotate {
    degrees: Value
}

enum MovementOption {
    RandomPosition,
    MousePointer,
}

enum Goto {
    Pos(GotoPos),
    Option(GotoOption),
}

struct GotoPos {
    x: Option<String>,
    y: Option<String>,
}

struct GotoOption {
    option: MovementOption,
}

enum Glide {
    Pos(GlidePos),
    Option(GlideOption),
}

struct GlidePos {
    x: Option<Value>,
    y: Option<Value>,
}

struct GlideOption {
    option: MovementOption,
}

enum Point {
    Direction(PointDirection),
    Towards(PointOption),
}

struct PointDirection {
    x: Option<Value>,
    y: Option<Value>,
}

struct PointOption {
    option: MovementOption,
}

struct ChangeX {
    x: Value
}

struct SetX {
    x: Value
}

struct ChangeY {
    y: Value
}

struct SetY {
    y: Value
}

struct IfOnEdgeBounce {}

enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}

struct SetRotationStyle {
    style: RotationStyle
}

struct XPosition;
struct YPosition;
struct Direction;

//
// Look blocks
//

struct SayForever {
    message: String,
}

struct Say {
    message: String,
    secs: Option<String>
}

struct Think {
    message: String,
    secs: Option<String>
}

struct SwitchCostume {
    costume: Option<String>,
}

struct NextCostume;

struct SwitchBackdrop {
    backdrop: Option<String>,
}

struct NextBackdrop;

struct ChangeSize {
    units: Option<Value>,
}

struct SetSize {
    percentage: Option<Value>,
}

struct ClearGraphicEffects;
struct ShowSprite;
struct HideSprite;

enum LayerOption {
    Front,
    Back,
}

struct GotoLayer {
    option: LayerOption
}

enum LayerDirection {
    Forward,
    Backward,
}

struct ChangeLayer {
    direction: LayerDirection,
    by: i32,
}

enum Costume {
    ByNumber(CostumeByNumber),
    ByName(CostumeByName)
}

struct CostumeByNumber;
struct CostumeByName;

enum Backdrop {
    ByNumber(BackdropByNumber),
    ByName(BackdropByName)
}

struct BackdropByNumber;
struct BackdropByName;

struct Size;

//
// Sound blocks
//

struct PlaySoundUntilDone {
    sound: String
}

struct StartSound {
    sound: String,
}

struct StopAllSounds;

enum SoundEffect {
    Pitch,
    Pan
}

struct ChangeEffectBy {
    effect: SoundEffect,
    units: Option<Value>,
}

struct SetEffectTo {
    effect: SoundEffect,
    percentage: Option<Value>,
}

struct ChangeVolumeBy {
    units: Option<Value>,
}

struct SetVolumeTo {
    percentage: Option<Value>,
}

struct Volume;

// Event blocks

struct WhenGreenFlagClicked;

enum Key {
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    Any,
    /// Any character from a-z and 0-9
    Alphanumerical(char),
}

struct WhenKeyClicked {
    key: Key,
}

struct WhenSpriteClicked;
struct WhenBackdropSwitchesTo {
    backdrop: String,
}

enum EventOption {
    Loudness,
    Timer
}

struct WhenOptionGreaterThen {
    option: EventOption,
    by: Option<Value>,
}

struct WhenIRecieveBroadcast {
    broadcast: String,
}

struct Broadcast {
    broadcast: String,
}

struct BroadcastAndWait {
    broadcast: String,
}

//
// Control
//

struct WaitSeconds {
    seconds: Option<Value>,
}

struct Repeat {
    units: Option<Value>,
}

struct Forever;

struct IfThen {
    condition: Option<String>,
    then: Option<String>,
}

struct IfThenElse {
    condition: Option<String>,
    then: Option<String>,
    otherwise: Option<String>,
}

struct WaitUntil {
    condition: Option<String>,
}

struct RepeatUntil {
    condition: Option<String>,
}

struct StopAll;

struct WhenIStartAsAClone;

enum SpriteOption {
    Myself,
    Sprite(String),
}

struct CreateCloneOf {
    of: SpriteOption,
}

struct DeleteClone;

// Sensing blocks

enum SensingOption {
    MousePointer,
    Edge,
    Sprite(String)
}

struct Touching {
    touching: SensingOption
}

struct TouchingColor {
    color: String,
}

struct ColorTouchingColor {
    color1: String,
    color2: String,
}

struct DistanceTo {
    to: SensingOption
}

struct Answer;

struct KeyPressed {
    key: Key
}

struct MouseDown;
struct MouseX;
struct MouseY;

enum DraggableOption {
    Draggable,
    NotDraggable
}

struct SetDragMode {
    option: DraggableOption,
}

struct Loudness;
struct Timer;

struct ResetTimer;

enum BackdropOfOption {
    BackdropNumber,
    BackdropName,
    Volume,
    MyVariable,
}

struct BackdropOf {
    backdrop: BackdropOfOption,
}

enum CurrentTimeOption {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second,
}

struct CurrentTime {
    option: CurrentTimeOption
}

struct DaysSince2000;
struct Username;

//
// Operators
//

struct Add {
    a: Value,
    b: Value,
}

struct Sub {
    a: Value,
    b: Value,
}

struct Mul {
    a: Value,
    b: Value,
}

struct Divide {
    a: Value,
    b: Value,
}

struct PickRandom {
    min: Value,
    max: Value,
}

struct GreaterThen {
    a: Value,
    b: Value,
}

struct LesserThen {
    a: Value,
    b: Value,
}

struct EqualTo {
    a: Value,
    b: Value,
}

struct And {
    a: Value,
    b: Value,
}

struct Or {
    a: Value,
    b: Value,
}

struct Not {
    a: Value,
}

struct Join {
    a: Value,
    b: Value,
}

struct LetterOf {
    index: Value,
    a: Value,
}

struct LengthOf {
    a: Value,
}

struct Contains {
    a: Value,
    b: Value,
}

struct Modulo {
    a: Value,
    b: Value,
}

struct Round {
    a: Value,
}

struct Absolute {
    a: Value,
}

// Variables

// FUCK.