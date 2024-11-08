use crate::{loader::Aseprite, FullyLoaded};
use aseprite_loader::binary::chunks::tags::AnimationDirection as RawDirection;
use bevy::prelude::*;
use std::{collections::VecDeque, ops::Range};

pub struct AsepriteAnimationPlugin;
impl Plugin for AsepriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvents>();
        app.add_systems(
            Update,
            (
                update_aseprite_ui_animation,
                update_aseprite_sprite_animation,
                load_animation_settings,
                hotreload_animations.run_if(on_event::<AssetEvent<Aseprite>>),
            ),
        );
        app.register_type::<AseSpriteAnimation>();
        app.register_type::<AseUiAnimation>();
        app.register_type::<Animation>();
        app.register_type::<AnimationState>();
        app.register_type::<PlayDirection>();
        app.register_type::<AnimationRepeat>();
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[require(Sprite, AnimationState)]
#[reflect]
pub struct AseSpriteAnimation {
    pub animation: Animation,
    pub aseprite: Handle<Aseprite>,
}

#[derive(Component, Reflect, Clone, Debug)]
#[require(UiImage, AnimationState)]
#[reflect]
pub struct AseUiAnimation {
    pub animation: Animation,
    pub aseprite: Handle<Aseprite>,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect]
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
    /// animation from tag
    pub fn tag(tag: &str) -> Self {
        Self::default().with_tag(tag)
    }

    /// animation speed multiplier, default is 1.0
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// animation with tag
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

    /// instanly starts playing a new animation, clearing any item left in the queue.
    pub fn play(&mut self, tag: &str, repeat: AnimationRepeat) {
        self.tag = Some(tag.to_string());
        self.repeat = repeat;
        self.queue.clear();
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
#[reflect]
pub struct AnimationState {
    /// carefull, changing the frame out of bounds
    /// may result in panic.
    pub current_frame: u16,
    pub elapsed: std::time::Duration,
    pub current_direction: PlayDirection,
}

#[allow(unused)]
impl AnimationState {
    pub fn current_frame(&self) -> u16 {
        self.current_frame
    }
}

#[derive(Default, Reflect)]
#[reflect]
pub enum PlayDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Event, Debug, Reflect)]
#[reflect]
pub enum AnimationEvents {
    Finished(Entity),
    LoopCycleFinished(Entity),
}

#[derive(Default, Clone, Reflect, Debug)]
#[reflect]
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

#[derive(Default, Debug, Clone, Reflect)]
#[reflect]
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

fn hotreload_animations(
    mut cmd: Commands,
    mut events: EventReader<AssetEvent<Aseprite>>,
    ui_animations: Query<(Entity, &AseUiAnimation), With<FullyLoaded>>,
    sprite_animations: Query<(Entity, &AseSpriteAnimation), With<FullyLoaded>>,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };

        ui_animations
            .iter()
            .filter(|(_, slice)| slice.aseprite.id() == *id)
            .for_each(|(entity, _)| {
                cmd.entity(entity).remove::<FullyLoaded>();
            });

        sprite_animations
            .iter()
            .filter(|(_, slice)| slice.aseprite.id() == *id)
            .for_each(|(entity, _)| {
                cmd.entity(entity).remove::<FullyLoaded>();
            });
    }
}

fn load_animation_settings(
    mut cmd: Commands,
    mut ui_animations: Query<(Entity, &mut AseUiAnimation), Without<FullyLoaded>>,
    mut sprite_animations: Query<(Entity, &mut AseSpriteAnimation), Without<FullyLoaded>>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (entity, mut animation) in ui_animations.iter_mut() {
        let Some(tag_str) = animation.animation.tag.as_ref() else {
            cmd.entity(entity).insert(FullyLoaded);
            continue;
        };

        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };

        cmd.entity(entity).insert(FullyLoaded);

        let Some(tag) = aseprite.tags.get(tag_str) else {
            continue;
        };

        animation.animation.direction = AnimationDirection::from(tag.direction);
    }

    for (entity, mut animation) in sprite_animations.iter_mut() {
        let Some(tag_str) = animation.animation.tag.as_ref() else {
            cmd.entity(entity).insert(FullyLoaded);
            continue;
        };

        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };

        cmd.entity(entity).insert(FullyLoaded);

        let Some(tag) = aseprite.tags.get(tag_str) else {
            continue;
        };

        animation.animation.direction = AnimationDirection::from(tag.direction);
    }
}

fn update_aseprite_sprite_animation(
    mut events: EventWriter<AnimationEvents>,
    mut animations: Query<(
        Entity,
        &mut AseSpriteAnimation,
        &mut AnimationState,
        &mut Sprite,
    )>,
    aseprites: Res<Assets<Aseprite>>,
    time: Res<Time>,
) {
    for (entity, mut animation, mut state, mut sprite) in animations.iter_mut() {
        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };

        let transition = update_animation_state(
            &mut animation.animation,
            &mut state,
            &aseprite,
            time.delta_secs(),
        );

        match transition {
            FrameTransition::AnimationFinished => {
                events.send(AnimationEvents::Finished(entity));
            }
            FrameTransition::AnimationLoopFinished => {
                events.send(AnimationEvents::LoopCycleFinished(entity));
            }
            _ => (),
        }

        sprite.image = aseprite.atlas_image.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: aseprite.get_atlas_index(usize::from(state.current_frame)),
        });
    }
}

fn update_aseprite_ui_animation(
    mut events: EventWriter<AnimationEvents>,
    mut animations: Query<(
        Entity,
        &mut AseUiAnimation,
        &mut AnimationState,
        &mut UiImage,
    )>,
    aseprites: Res<Assets<Aseprite>>,
    time: Res<Time>,
) {
    for (entity, mut animation, mut state, mut image) in animations.iter_mut() {
        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };

        let transition = update_animation_state(
            &mut animation.animation,
            &mut state,
            &aseprite,
            time.delta_secs(),
        );

        match transition {
            FrameTransition::AnimationFinished => {
                events.send(AnimationEvents::Finished(entity));
            }
            FrameTransition::AnimationLoopFinished => {
                events.send(AnimationEvents::LoopCycleFinished(entity));
            }
            _ => (),
        }

        image.image = aseprite.atlas_image.clone();
        image.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: aseprite.get_atlas_index(usize::from(state.current_frame)),
        });
    }
}

fn update_animation_state(
    animation: &mut Animation,
    state: &mut AnimationState,
    aseprite: &Aseprite,
    delta_secs: f32,
) -> FrameTransition {
    let maybe_tag = animation
        .tag
        .as_ref()
        .map(|t| aseprite.tags.get(t))
        .flatten();

    let range = maybe_tag
        .map(|t| *t.range.start()..*t.range.end())
        .unwrap_or(0..aseprite.frame_durations.len() as u16);

    state.elapsed += std::time::Duration::from_secs_f32(delta_secs);

    let Some(frame_duration) = aseprite
        .frame_durations
        .get(usize::from(state.current_frame))
    else {
        return FrameTransition::None;
    };

    if state.elapsed > *frame_duration {
        let transition = next_frame(state, animation, range);
        if let FrameTransition::AnimationFinished = transition {
            match animation.queue.pop_front() {
                Some((tag, repeat)) => {
                    animation.tag = Some(tag);
                    animation.repeat = repeat;
                }
                None => {
                    animation.playing = false;
                }
            };
        }
        state.elapsed = std::time::Duration::ZERO;
        return transition;
    }

    return FrameTransition::None;
}

#[derive(Debug)]
enum FrameTransition {
    None,
    AnimationFinished,
    AnimationLoopFinished,
}

fn next_frame(
    state: &mut AnimationState,
    animation: &mut Animation,
    animation_range: Range<u16>,
) -> FrameTransition {
    match animation.direction {
        AnimationDirection::Forward => {
            let next = state.current_frame + 1;
            if next > animation_range.end {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_frame = animation_range.start;
                        return FrameTransition::AnimationLoopFinished;
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_frame = animation_range.start;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return FrameTransition::AnimationFinished;
                        }
                    }
                }
            } else {
                state.current_frame = next;
            }
        }
        AnimationDirection::Reverse => {
            let next = state
                .current_frame
                .checked_sub(1)
                .unwrap_or(animation_range.end);

            if next == animation_range.end {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_frame = animation_range.end - 1;
                        return FrameTransition::AnimationLoopFinished;
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_frame = animation_range.end - 1;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return FrameTransition::AnimationFinished;
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
                        state.current_frame = animation_range.end - 2;
                        return FrameTransition::AnimationLoopFinished;
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_direction = PlayDirection::Backward;
                            state.current_frame = animation_range.end - 2;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return FrameTransition::AnimationFinished;
                        }
                    }
                };
            } else if next <= animation_range.start && !is_forward {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_direction = PlayDirection::Forward;
                        state.current_frame = animation_range.start;
                        return FrameTransition::AnimationLoopFinished;
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_direction = PlayDirection::Forward;
                            state.current_frame = animation_range.start;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            return FrameTransition::AnimationFinished;
                        }
                    }
                };
            } else {
                state.current_frame = next;
            }
        }
    };
    FrameTransition::None
}
