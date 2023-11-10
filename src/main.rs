use std::{collections::HashMap, error::Error};

use blocks::{Block, BlockType, WhenGreenFlagClicked};

use crate::blocks::BlockType::*;

pub(crate) mod block_defs;
pub mod block_names;
pub mod blocks;
pub mod decomp;

fn main() -> Result<(), Box<dyn Error>> {
    let project = decomp::Project::new(None)?;

    for sprite in project {
        println!("{}", sprite.name);

        let blocks = sprite.blocks;
        for block in &blocks {
            match block.1 {
                WhenGreenFlagClicked(a) => {
                    println!("main:");
                    follow_main_block(&blocks, a);
                }
                WhenKeyPressed(a) => {
                    println!("when {:?} pressed:", a.key);
                    follow_main_block(&blocks, a);
                }
                WhenSpriteClicked(a) => {
                    println!("when sprite clicked:");
                    follow_main_block(&blocks, a);
                }
                WhenStageClicked(a) => {
                    println!("when stage clicked:");
                    follow_main_block(&blocks, a);
                }
                WhenBackdropSwitchesTo(a) => {
                    println!("when backdrop switched to {:?}:", a.backdrop);
                    follow_main_block(&blocks, a);
                }
                WhenOptionGreaterThen(a) => {
                    println!("when {:?} greater then {:?}", a.option, a.by);
                    follow_main_block(&blocks, a);
                }
                WhenIRecieveBroadcast(a) => {
                    println!("when i recieve {:?}:", a.broadcast);
                    follow_main_block(&blocks, a);
                }
                Broadcast(a) => {
                    println!("broadcast {:?}:", a.broadcast);
                    follow_main_block(&blocks, a);
                }
                BroadcastAndWait(a) => {
                    println!("broadcast+wait {:?}:", a.broadcast);
                    follow_main_block(&blocks, a);
                }
                ProceduresDefinition(a) => {
                    println!("procedure:");
                    follow_main_block(&blocks, a);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn follow_main_block(blocks: &HashMap<String, BlockType>, b: &dyn Block) {
    if let Some(m) = &b.next() {
        if let Some(next) = blocks.get(&m.replace("\"", "")) {
            println!("\t{:?}", next);
            match next {
                Move(a) => follow_main_block(blocks, a),
                RotateLeft(a) => follow_main_block(blocks, a),
                RotateRight(a) => follow_main_block(blocks, a),
                Goto(a) => match a {
                    blocks::Goto::Pos(b) => follow_main_block(blocks, b),
                    blocks::Goto::Option(b) => follow_main_block(blocks, b),
                },
                Glide(a) => match a {
                    blocks::Glide::Pos(b) => follow_main_block(blocks, b),
                    blocks::Glide::Option(b) => follow_main_block(blocks, b),
                },
                Point(a) => match a {
                    blocks::Point::Direction(b) => follow_main_block(blocks, b),
                    blocks::Point::Towards(b) => follow_main_block(blocks, b),
                },
                ChangeX(a) => follow_main_block(blocks, a),
                SetX(a) => follow_main_block(blocks, a),
                ChangeY(a) => follow_main_block(blocks, a),
                SetY(a) => follow_main_block(blocks, a),
                IfOnEdgeBounce(a) => follow_main_block(blocks, a),
                SetRotationStyle(a) => follow_main_block(blocks, a),
                XPosition(a) => follow_main_block(blocks, a),
                YPosition(a) => follow_main_block(blocks, a),
                Direction(a) => follow_main_block(blocks, a),
                Say(a) => follow_main_block(blocks, a),
                SayForever(a) => follow_main_block(blocks, a),
                Think(a) => follow_main_block(blocks, a),
                ThinkForever(a) => follow_main_block(blocks, a),
                SwitchCostume(a) => follow_main_block(blocks, a),
                NextCostume(a) => follow_main_block(blocks, a),
                SwitchBackdrop(a) => follow_main_block(blocks, a),
                SwitchBackdropAndWait(a) => follow_main_block(blocks, a),
                NextBackdrop(a) => follow_main_block(blocks, a),
                ChangeSize(a) => follow_main_block(blocks, a),
                SetSize(a) => follow_main_block(blocks, a),
                ClearGraphicEffects(a) => follow_main_block(blocks, a),
                ShowSprite(a) => follow_main_block(blocks, a),
                HideSprite(a) => follow_main_block(blocks, a),
                HideAllSprites(a) => follow_main_block(blocks, a),
                GotoLayer(a) => follow_main_block(blocks, a),
                ChangeLayer(a) => follow_main_block(blocks, a),
                Costume(a) => {}
                Backdrop(a) => {}
                Size(a) => follow_main_block(blocks, a),
                PlaySound(a) => follow_main_block(blocks, a),
                PlaySoundUntilDone(a) => follow_main_block(blocks, a),
                StartSound(a) => follow_main_block(blocks, a),
                StopAllSounds(a) => follow_main_block(blocks, a),
                ChangeEffectBy(a) => follow_main_block(blocks, a),
                SetEffectTo(a) => follow_main_block(blocks, a),
                ClearSoundEffects(a) => follow_main_block(blocks, a),
                ChangeVolumeBy(a) => follow_main_block(blocks, a),
                SetVolumeTo(a) => follow_main_block(blocks, a),
                Volume(a) => follow_main_block(blocks, a),
                BlockType::WhenGreenFlagClicked(a) => follow_main_block(blocks, a),
                WhenKeyPressed(a) => follow_main_block(blocks, a),
                WhenSpriteClicked(a) => follow_main_block(blocks, a),
                WhenStageClicked(a) => follow_main_block(blocks, a),
                WhenBackdropSwitchesTo(a) => follow_main_block(blocks, a),
                WhenOptionGreaterThen(a) => follow_main_block(blocks, a),
                WhenIRecieveBroadcast(a) => follow_main_block(blocks, a),
                Broadcast(a) => follow_main_block(blocks, a),
                BroadcastAndWait(a) => follow_main_block(blocks, a),
                WaitSeconds(a) => follow_main_block(blocks, a),
                Repeat(a) => follow_main_block(blocks, a),
                Forever(a) => follow_main_block(blocks, a),
                IfThen(a) => follow_main_block(blocks, a),
                IfThenElse(a) => follow_main_block(blocks, a),
                WaitUntil(a) => follow_main_block(blocks, a),
                RepeatUntil(a) => follow_main_block(blocks, a),
                StopAll(a) => follow_main_block(blocks, a),
                WhenIStartAsAClone(a) => follow_main_block(blocks, a),
                CreateCloneOf(a) => follow_main_block(blocks, a),
                DeleteClone(a) => follow_main_block(blocks, a),
                Touching(a) => follow_main_block(blocks, a),
                TouchingMenu(a) => follow_main_block(blocks, a),
                TouchingColor(a) => follow_main_block(blocks, a),
                ColorTouchingColor(a) => follow_main_block(blocks, a),
                DistanceTo(a) => follow_main_block(blocks, a),
                Answer(a) => follow_main_block(blocks, a),
                KeyPressed(a) => follow_main_block(blocks, a),
                MouseDown(a) => follow_main_block(blocks, a),
                MouseX(a) => follow_main_block(blocks, a),
                MouseY(a) => follow_main_block(blocks, a),

                SetDragMode(a) => follow_main_block(blocks, a),
                Loudness(a) => follow_main_block(blocks, a),
                Timer(a) => follow_main_block(blocks, a),
                ResetTimer(a) => follow_main_block(blocks, a),
                BackdropOf(a) => follow_main_block(blocks, a),
                CurrentTime(a) => follow_main_block(blocks, a),
                DaysSince2000(a) => follow_main_block(blocks, a),
                Username(a) => follow_main_block(blocks, a),
                Add(a) => follow_main_block(blocks, a),
                Sub(a) => follow_main_block(blocks, a),
                Mul(a) => follow_main_block(blocks, a),
                Divide(a) => follow_main_block(blocks, a),
                PickRandom(a) => follow_main_block(blocks, a),
                GreaterThen(a) => follow_main_block(blocks, a),
                LesserThen(a) => follow_main_block(blocks, a),
                EqualTo(a) => follow_main_block(blocks, a),
                And(a) => follow_main_block(blocks, a),
                Or(a) => follow_main_block(blocks, a),
                Not(a) => follow_main_block(blocks, a),
                Join(a) => follow_main_block(blocks, a),
                LetterOf(a) => follow_main_block(blocks, a),
                LengthOf(a) => follow_main_block(blocks, a),
                Contains(a) => follow_main_block(blocks, a),
                Modulo(a) => follow_main_block(blocks, a),
                Round(a) => follow_main_block(blocks, a),
                Absolute(a) => follow_main_block(blocks, a),
                SoundEffectsMenu(a) => follow_main_block(blocks, a),
                SoundSoundsMenu(a) => follow_main_block(blocks, a),
                PointTowardsMenu(a) => follow_main_block(blocks, a),
                UnusedOpcode(a) => follow_main_block(blocks, a),
                InvalidOpcode(a) => follow_main_block(blocks, a),
                DraggableOption(_) => {}
                ProceduresCall(a) => follow_main_block(blocks, a),
                ProceduresDeclaration(a) => follow_main_block(blocks, a),
                ProceduresDefinition(a) => follow_main_block(blocks, a),
                ProceduresPrototype(a) => follow_main_block(blocks, a),
                _ => {}
            };
        }
    }
}
