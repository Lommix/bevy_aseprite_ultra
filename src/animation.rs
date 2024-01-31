use std::ops::Range;

use crate::loader::Aseprite;
use bevy::prelude::*;
use sprity::aseprite::binary::chunks::tags::AnimationDirection as RawDirection;

pub struct AsepriteAnimationPlugin;
impl Plugin for AsepriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvents>();
        app.add_systems(
            Update,
            (insert_aseprite_animation, update_aseprite_animation).chain(),
        );
    }
}

/// The `AsepriteAnimationBundle` bundles the components needed to render an animation.
/// ```rust
/// // example from examples/animation.rs
/// command.spawn(AsepriteAnimationBundle {
///     aseprite: server.load("player.aseprite"),
///     transform: Transform::from_translation(Vec3::new(15., -20., 0.)),
///     animation: Animation::default().with_tag("walk-right"),
///     sprite: Sprite {
///         flip_x: true,
///         ..default()
///     },
///     ..default()
/// })
/// ```
/// If a tag is present `repeat` and `direction` in `AnimationControl` will be overwritten by the values
/// porvided in the aseprite file, but can be interacted with at runtime.
#[derive(Bundle, Default)]
pub struct AsepriteAnimationBundle {
    pub aseprite: Handle<Aseprite>,
    pub animation: Animation,
    pub animation_state: AnimationState,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Component)]
pub struct Animation {
    tag: Option<String>,
    speed: f32,
    playing: bool,
    repeat: AnimationRepeat,
    direction: AnimationDirection,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            tag: None,
            speed: 1.0,
            playing: false,
            repeat: AnimationRepeat::Loop,
            direction: AnimationDirection::Forward,
        }
    }
}

impl Animation {
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }
    pub fn with_repeat(mut self, repeat: AnimationRepeat) -> Self {
        self.repeat = repeat;
        self
    }
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl From<&str> for Animation {
    fn from(tag: &str) -> Self {
        Animation {
            tag: Some(tag.to_string()),
            speed: 1.0,
            ..Default::default()
        }
    }
}

#[derive(Component, Default)]
pub struct AnimationState {
    current_frame: usize,
    elapsed: std::time::Duration,
    current_direction: PlayDirection,
}

impl AnimationState {
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
}

#[derive(Default)]
enum PlayDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Event, Debug)]
pub enum AnimationEvents {
    Finished(Entity),
    LoopCycleFinished(Entity),
}

#[derive(Default)]
pub enum AnimationDirection {
    #[default]
    Forward,
    Reverse,
    PingPong,
    PingPongReverse,
}

impl From<RawDirection> for AnimationDirection {
    fn from(direction: RawDirection) -> AnimationDirection {
        match direction {
            RawDirection::Forward => AnimationDirection::Forward,
            RawDirection::Reverse => AnimationDirection::Reverse,
            RawDirection::PingPong => AnimationDirection::PingPong,
            RawDirection::PingPongReverse => AnimationDirection::PingPongReverse,
            _ => panic!("Invalid AnimationDirection"),
        }
    }
}

#[derive(Default, Component)]
pub enum AnimationRepeat {
    #[default]
    Loop,
    Count(u32),
}

impl From<u16> for AnimationRepeat {
    fn from(value: u16) -> Self {
        match value {
            0 => AnimationRepeat::Loop,
            n => AnimationRepeat::Count(u32::from(n)),
        }
    }
}

fn insert_aseprite_animation(
    mut query: Query<
        (
            Entity,
            &mut Sprite,
            &mut AnimationState,
            &mut Animation,
            &Handle<Aseprite>,
        ),
        Without<Handle<Image>>,
    >,
    mut cmd: Commands,
    asesprites: Res<Assets<Aseprite>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    query.iter_mut().for_each(
        |(entity, mut sprite, mut state, mut control, aseprite_handle)| {
            let Some(aseprite) = asesprites.get(aseprite_handle) else {
                return;
            };

            let Some(atlas_handle) = aseprite.atlas.as_ref() else {
                return;
            };

            let Some(atlas) = atlases.get(atlas_handle) else {
                return;
            };

            let maybe_tag = match control.tag.as_ref() {
                Some(tag) => Some(
                    aseprite.tags.get(tag).expect(
                        format!(
                            "animation tag '{}' not found in '{:?}'",
                            tag,
                            aseprite_handle.path()
                        )
                        .as_str(),
                    ),
                ),
                None => None,
            };

            let start_frame_index = usize::from(maybe_tag.map(|tag| tag.range.start).unwrap_or(0));
            let end_frame_index = usize::from(
                maybe_tag
                    .map(|tag| tag.range.end - 1)
                    .unwrap_or(aseprite.frame_durations.len() as u16 - 1),
            );

            // let (start_index, end_index) = match control.tag {
            //     Some(tag) => {}
            //     None => (0, aseprite.frame_durations.len() - 1),
            // };

            state.current_frame = start_frame_index;
            state.elapsed = std::time::Duration::ZERO;
            control.playing = true;

            cmd.entity(entity).insert(atlas.texture.clone());

            if let Some(tag) = maybe_tag {
                control.repeat = AnimationRepeat::from(tag.repeat);
                control.direction = AnimationDirection::from(tag.direction);
                state.current_direction = match control.direction {
                    AnimationDirection::Reverse | AnimationDirection::PingPongReverse => {
                        state.current_frame = end_frame_index;
                        PlayDirection::Backward
                    }
                    _ => PlayDirection::Forward,
                };
            }

            let atlas_frame_index = aseprite.get_atlas_index(state.current_frame);
            sprite.rect = Some(atlas.textures[atlas_frame_index]);
        },
    );
}

fn update_aseprite_animation(
    mut query: Query<(
        Entity,
        &mut AnimationState,
        &mut Sprite,
        &mut Animation,
        &Handle<Aseprite>,
    )>,
    mut events: EventWriter<AnimationEvents>,
    asesprites: Res<Assets<Aseprite>>,
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(
        |(entity, mut state, mut sprite, mut control, aseprite_handle)| {
            if !control.playing {
                return;
            }

            let Some(aseprite) = asesprites.get(aseprite_handle) else {
                return;
            };

            let Some(atlas_handle) = aseprite.atlas.as_ref() else {
                return;
            };

            let Some(atlas) = atlases.get(atlas_handle) else {
                return;
            };

            let animation_range = match control.tag.as_ref() {
                Some(tag) => {
                    let r = &aseprite.tags.get(tag).as_ref().unwrap().range;
                    usize::from(r.start)..usize::from(r.end)
                }
                None => 0..aseprite.frame_durations.len(),
            };

            state.elapsed +=
                std::time::Duration::from_secs_f32(time.delta_seconds() * control.speed);

            let Some(frame_time) = aseprite.frame_durations.get(state.current_frame) else {
                return;
            };

            let atlas_frame_index = aseprite.get_atlas_index(state.current_frame);
            sprite.rect = Some(atlas.textures[atlas_frame_index]);

            if state.elapsed > *frame_time {
                match next_frame(&mut state, &mut control, &animation_range) {
                    Some(FrameTransition::AnimationFinished) => {
                        // mut just because of this?
                        control.playing = false;
                        events.send(AnimationEvents::Finished(entity));
                        return;
                    }
                    Some(FrameTransition::AnimationLoopFinished) => {
                        events.send(AnimationEvents::LoopCycleFinished(entity));
                    }
                    None => {}
                }

                state.elapsed = std::time::Duration::ZERO;
            }
        },
    );
}

enum FrameTransition {
    AnimationFinished,
    AnimationLoopFinished,
}

fn next_frame(
    state: &mut AnimationState,
    animation: &mut Animation,
    animation_range: &Range<usize>,
) -> Option<FrameTransition> {
    match animation.direction {
        AnimationDirection::Forward => {
            let next = state.current_frame + 1;
            if next >= animation_range.end {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_frame = animation_range.start;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_frame = animation_range.start;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                }
            } else {
                state.current_frame = next;
            }
        }
        AnimationDirection::Reverse => {
            let next = state.current_frame.checked_sub(1).unwrap_or(0);
            if next < animation_range.start {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_frame = animation_range.end - 1;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_frame = animation_range.end - 1;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                }
            } else {
                state.current_frame = next;
            }
        }
        AnimationDirection::PingPong | AnimationDirection::PingPongReverse => {
            let next = match state.current_direction {
                PlayDirection::Forward => state.current_frame + 1,
                PlayDirection::Backward => state.current_frame.checked_sub(1).unwrap_or(0),
            };

            let is_forward = match state.current_direction {
                PlayDirection::Forward => true,
                PlayDirection::Backward => false,
            };

            if next >= animation_range.end && is_forward {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_direction = PlayDirection::Backward;
                        state.current_frame = animation_range.end - 1;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_direction = PlayDirection::Backward;
                            state.current_frame = animation_range.end - 1;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                };
            } else if next <= animation_range.start && !is_forward {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_direction = PlayDirection::Forward;
                        state.current_frame = animation_range.start;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_direction = PlayDirection::Forward;
                            state.current_frame = animation_range.start;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                };
            } else {
                state.current_frame = next;
            }
        }
    };
    None
}
