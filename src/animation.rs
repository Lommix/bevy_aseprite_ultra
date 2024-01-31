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

/// The `AsepriteAnimationBundle` bundles the components needed to render an animation of an aseprite.
/// ```rust
/// // example from examples/animation.rs
/// command.spawn(AsepriteAnimationBundle {
///     aseprite: server.load("player.aseprite"),
///     transform: Transform::from_translation(Vec3::new(15., -20., 0.)),
///     animation_speed: AnimationSpeed(0.5),
///     sprite: Sprite {
///         flip_x: true,
///         ..default()
///     },
///     ..default()
/// })
/// ```
/// `animation_frame`, `animation_repeat`, `animation_direction` and `animation_state` will be
/// loaded from the aseprite file, but can be interacted with while running.
/// Beware, that changes get lost when reloading the aseprite file.
#[derive(Bundle, Default)]
pub struct AsepriteAnimationBundle {
    pub aseprite: Handle<Aseprite>,
    pub animation_frame: AnimationFrame,
    pub animation_speed: AnimationSpeed,
    pub animation_repeat: AnimationRepeat,
    pub animation_direction: AnimationDirection,
    pub animation_state: AnimationState,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Component, Default)]
pub struct AnimationFrame {
    pub index: usize,
    elapsed: std::time::Duration,
    current_direction: PlayDirection,
}

#[derive(Component, Default)]
pub enum AnimationState {
    #[default]
    Playing,
    Stoped,
}

#[derive(Default)]
enum PlayDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Component)]
pub struct AnimationSpeed(pub f32);
impl Default for AnimationSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Default)]
pub struct AnimationTag(String);
impl From<&str> for AnimationTag {
    fn from(value: &str) -> AnimationTag {
        AnimationTag(value.to_string())
    }
}

#[derive(Event, Debug)]
pub enum AnimationEvents {
    Finished(Entity),
    LoopCycleFinished(Entity),
}

#[derive(Default, Component)]
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
            &mut AnimationDirection,
            &mut AnimationRepeat,
            &mut AnimationFrame,
            &mut AnimationState,
            &Handle<Aseprite>,
        ),
        Without<Handle<Image>>,
    >,
    mut cmd: Commands,
    tag_query: Query<&AnimationTag>,
    asesprites: Res<Assets<Aseprite>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    query.iter_mut().for_each(
        |(entity, mut sprite, mut direction, mut repeat, mut frame, mut state, aseprite_handle)| {
            let Some(aseprite) = asesprites.get(aseprite_handle) else {
                return;
            };

            let Some(atlas_handle) = aseprite.atlas.as_ref() else {
                return;
            };

            let Some(atlas) = atlases.get(atlas_handle) else {
                return;
            };

            let maybe_animation = tag_query.get(entity).ok();
            let maybe_tag = match maybe_animation {
                Some(animation) => Some(
                    aseprite.tags.get(&animation.0).expect(
                        format!(
                            "animation tag '{}' not found in '{:?}'",
                            animation.0,
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

            frame.index = start_frame_index;
            frame.elapsed = std::time::Duration::ZERO;
            *state = AnimationState::Playing;

            cmd.entity(entity).insert(atlas.texture.clone());

            if let Some(tag) = maybe_tag {
                *repeat = AnimationRepeat::from(tag.repeat);
                *direction = AnimationDirection::from(tag.direction);
                frame.current_direction = match *direction {
                    AnimationDirection::Reverse | AnimationDirection::PingPongReverse => {
                        frame.index = end_frame_index;
                        PlayDirection::Backward
                    }
                    _ => PlayDirection::Forward,
                };
            }

            let atlas_frame_index = aseprite.get_atlas_index(frame.index);
            sprite.rect = Some(atlas.textures[atlas_frame_index]);
        },
    );
}

fn update_aseprite_animation(
    mut query: Query<(
        Entity,
        &mut AnimationFrame,
        &mut Sprite,
        &mut AnimationRepeat,
        &mut AnimationState,
        &AnimationSpeed,
        &AnimationDirection,
        &Handle<Aseprite>,
    )>,
    mut events: EventWriter<AnimationEvents>,
    tag_query: Query<&AnimationTag>,
    asesprites: Res<Assets<Aseprite>>,
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(
        |(
            entity,
            mut frame,
            mut sprite,
            mut repeat,
            mut state,
            speed,
            direction,
            aseprite_handle,
        )| {
            if let AnimationState::Stoped = *state {
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

            let tag = tag_query.get(entity).ok();

            let animation_range = match tag {
                Some(animation) => {
                    let r = &aseprite.tags.get(&animation.0).as_ref().unwrap().range;
                    usize::from(r.start)..usize::from(r.end)
                }
                None => 0..aseprite.frame_durations.len(),
            };

            frame.elapsed += std::time::Duration::from_secs_f32(time.delta_seconds() * speed.0);

            let Some(frame_time) = aseprite.frame_durations.get(frame.index) else {
                return;
            };

            let atlas_frame_index = aseprite.get_atlas_index(frame.index);
            sprite.rect = Some(atlas.textures[atlas_frame_index]);

            if frame.elapsed > *frame_time {
                match next_frame(&mut frame, &mut repeat, direction, &animation_range) {
                    Some(FrameTransition::AnimationFinished) => {
                        *state = AnimationState::Stoped;
                        events.send(AnimationEvents::Finished(entity));
                        return;
                    }
                    Some(FrameTransition::AnimationLoopFinished) => {
                        events.send(AnimationEvents::LoopCycleFinished(entity));
                    }
                    None => {}
                }

                frame.elapsed = std::time::Duration::ZERO;
            }
        },
    );
}

enum FrameTransition {
    AnimationFinished,
    AnimationLoopFinished,
}

fn next_frame(
    frame: &mut AnimationFrame,
    repeat: &mut AnimationRepeat,
    direction: &AnimationDirection,
    animation_range: &Range<usize>,
) -> Option<FrameTransition> {
    match *direction {
        AnimationDirection::Forward => {
            let next = frame.index + 1;
            if next >= animation_range.end {
                match *repeat {
                    AnimationRepeat::Loop => {
                        frame.index = animation_range.start;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            frame.index = animation_range.start;
                            *repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                }
            } else {
                frame.index = next;
            }
        }
        AnimationDirection::Reverse => {
            let next = frame.index.checked_sub(1).unwrap_or(0);
            if next < animation_range.start {
                match *repeat {
                    AnimationRepeat::Loop => {
                        frame.index = animation_range.end - 1;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            frame.index = animation_range.end - 1;
                            *repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                }
            } else {
                frame.index = next;
            }
        }
        AnimationDirection::PingPong | AnimationDirection::PingPongReverse => {
            let next = match frame.current_direction {
                PlayDirection::Forward => frame.index + 1,
                PlayDirection::Backward => frame.index.checked_sub(1).unwrap_or(0),
            };

            let is_forward = match frame.current_direction {
                PlayDirection::Forward => true,
                PlayDirection::Backward => false,
            };

            //wrong!!
            if next >= animation_range.end && is_forward {
                match *repeat {
                    AnimationRepeat::Loop => {
                        frame.current_direction = PlayDirection::Backward;
                        frame.index = animation_range.end - 1;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            frame.current_direction = PlayDirection::Backward;
                            frame.index = animation_range.end - 1;
                            *repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                };
            } else if next <= animation_range.start && !is_forward {
                match *repeat {
                    AnimationRepeat::Loop => {
                        frame.current_direction = PlayDirection::Forward;
                        frame.index = animation_range.start;
                        return Some(FrameTransition::AnimationLoopFinished);
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            frame.current_direction = PlayDirection::Forward;
                            frame.index = animation_range.start;
                            *repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return Some(FrameTransition::AnimationFinished);
                        }
                    }
                };
            } else {
                frame.index = next;
            }
        }
    };
    None
}
