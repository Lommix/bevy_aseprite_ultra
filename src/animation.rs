use std::{collections::VecDeque, ops::Range};

use crate::{loader::Aseprite, NotLoaded, UiTag};
use aseprite_loader::binary::chunks::tags::AnimationDirection as RawDirection;
use bevy::prelude::*;

pub struct AsepriteAnimationPlugin;
impl Plugin for AsepriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvents>();
        app.add_systems(
            Update,
            (insert_aseprite_animation, update_aseprite_animation).chain(),
        );

        app.register_type::<Animation>();
        app.register_type::<AnimationState>();
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
    pub atlas: TextureAtlas,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub not_loaded: NotLoaded,
}

#[derive(Bundle, Default)]
pub struct AsepriteAnimationUiBundle {
    pub aseprite: Handle<Aseprite>,
    pub animation: Animation,
    pub animation_state: AnimationState,
    pub atlas: TextureAtlas,
    pub ui_tag: UiTag,
    pub not_loaded: NotLoaded,
}

#[derive(Component, Reflect)]
pub struct Animation {
    pub tag: Option<String>,
    pub speed: f32,
    pub playing: bool,
    pub repeat: AnimationRepeat,
    pub direction: AnimationDirection,
    pub queue: VecDeque<(String, AnimationRepeat)>,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            tag: None,
            speed: 1.0,
            playing: false,
            repeat: AnimationRepeat::Loop,
            direction: AnimationDirection::Forward,
            queue: VecDeque::new(),
        }
    }
}

impl Animation {
    /// animation speed multiplier, default is 1.0
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// provide a tag string. Panics at runtime, if animation is not found
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    /// sets a repeat count, defaults is loop
    pub fn with_repeat(mut self, repeat: AnimationRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    /// provides an animation direction, maybe overwritten by aseprite tag
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// chains an animation after the current one is done
    pub fn with_then(mut self, tag: &str, repeats: AnimationRepeat) -> Self {
        self.queue.push_back((tag.to_string(), repeats));
        self
    }

    /// chains an animation after the current one is done
    pub fn then(&mut self, tag: &str, repeats: AnimationRepeat) {
        self.queue.push_back((tag.to_string(), repeats));
    }

    /// clears any queued up animations
    pub fn clear_queue(&mut self) {
        self.queue.clear()
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

#[derive(Component, Default, Reflect)]
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

#[derive(Default, Reflect)]
enum PlayDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Event, Debug, Reflect)]
pub enum AnimationEvents {
    Finished(Entity),
    LoopCycleFinished(Entity),
}

#[derive(Default, Reflect)]
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

#[derive(Default, Component, Reflect)]
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
            &mut AnimationState,
            &mut Animation,
            &mut TextureAtlas,
            &Handle<Aseprite>,
            Option<&UiTag>,
        ),
        With<NotLoaded>,
    >,
    mut cmd: Commands,
    asesprites: Res<Assets<Aseprite>>,
) {
    query.iter_mut().for_each(
        |(entity, mut state, mut control, mut atlas, aseprite_handle, maybe_ui)| {
            let Some(aseprite) = asesprites.get(aseprite_handle) else {
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

            state.current_frame = start_frame_index;
            state.elapsed = std::time::Duration::ZERO;
            control.playing = true;

            if let Some(tag) = maybe_tag {
                control.direction = AnimationDirection::from(tag.direction);
                state.current_direction = match control.direction {
                    AnimationDirection::Reverse | AnimationDirection::PingPongReverse => {
                        state.current_frame = end_frame_index;
                        PlayDirection::Backward
                    }
                    _ => PlayDirection::Forward,
                };
            } else {
                match control.direction {
                    AnimationDirection::Reverse | AnimationDirection::PingPongReverse => {
                        state.current_frame = end_frame_index;
                    }
                    _ => (),
                };
            }

            atlas.layout = aseprite.atlas_layout.clone();
            atlas.index = aseprite.get_atlas_index(state.current_frame);

            if let Some(mut cmd) = cmd.get_entity(entity) {
                match maybe_ui {
                    Some(_) => {
                        cmd.remove::<NotLoaded>()
                            .insert(UiImage::new(aseprite.atlas_image.clone()));
                    }
                    None => {
                        cmd.remove::<NotLoaded>()
                            .insert(aseprite.atlas_image.clone());
                    }
                };
            };
        },
    );
}

fn update_aseprite_animation(
    mut query: Query<(
        Entity,
        &mut AnimationState,
        &mut TextureAtlas,
        &mut Animation,
        &Handle<Aseprite>,
    )>,
    mut events: EventWriter<AnimationEvents>,
    asesprites: Res<Assets<Aseprite>>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(
        |(entity, mut state, mut atlas_comp, mut animation, aseprite_handle)| {
            if !animation.playing {
                return;
            }

            let Some(aseprite) = asesprites.get(aseprite_handle) else {
                return;
            };

            let animation_range = match animation.tag.as_ref() {
                Some(tag) => {
                    let r = &aseprite.tags.get(tag).as_ref().unwrap().range;
                    usize::from(r.start)..usize::from(r.end)
                }
                None => 0..aseprite.frame_durations.len(),
            };

            state.elapsed +=
                std::time::Duration::from_secs_f32(time.delta_seconds() * animation.speed);

            let Some(frame_time) = aseprite.frame_durations.get(state.current_frame) else {
                return;
            };

            let atlas_frame_index = aseprite.get_atlas_index(state.current_frame);
            atlas_comp.index = atlas_frame_index;

            if state.elapsed > *frame_time {
                match next_frame(&mut state, &mut animation, &animation_range) {
                    Some(FrameTransition::AnimationFinished) => {
                        // mut just because of this? @fix someday
                        match animation.queue.pop_front() {
                            Some((tag, repeat)) => {
                                animation.tag = Some(tag);
                                animation.repeat = repeat;
                            }
                            None => {
                                animation.playing = false;
                                events.send(AnimationEvents::Finished(entity));
                            }
                        }
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
