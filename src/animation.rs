use crate::loader::Aseprite;
use aseprite_loader::binary::chunks::tags::AnimationDirection as RawDirection;
use bevy::prelude::*;
use std::{collections::VecDeque, time::Duration};

pub struct AsepriteAnimationPlugin;
impl Plugin for AsepriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvents>();
        app.add_event::<NextFrameEvent>();
        app.add_systems(
            Update,
            (
                update_aseprite_animation,
                // hotreload_animation.run_if(on_event::<AssetEvent<Aseprite>>),
            ),
        );
        app.add_animation_render_system((render_image_node, render_sprite));

        app.add_observer(next_frame);

        app.register_type::<AseAnimation>();
        app.register_type::<Animation>();
        app.register_type::<AnimationState>();
        app.register_type::<PlayDirection>();
        app.register_type::<AnimationRepeat>();
    }
}

pub trait AddAnimationRenderSystem {
    fn add_animation_render_system<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self;
}

impl AddAnimationRenderSystem for App {
    fn add_animation_render_system<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
        self.add_systems(
            Update,
            systems.after(update_aseprite_animation), // .before(remove_fully_loaded_animation),
        );
        self
    }
}

/// Create a Component using an Aseprite Animation.
#[derive(Component, Default, Reflect, Clone, Debug)]
#[require(AnimationState)]
#[reflect]
pub struct AseAnimation {
    pub animation: Animation,
    pub aseprite: Handle<Aseprite>,
}

fn render_image_node(
    mut animations: Query<
        (&AseAnimation, &mut ImageNode, &AnimationState),
        // Or<(With<FullyLoadedAnimation>, Changed<AseAnimation>)>,
    >,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (animation, mut target, state) in &mut animations {
        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };
        target.image = aseprite.atlas_image.clone();
        target.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: aseprite.get_atlas_index(usize::from(state.current_frame)),
        });
    }
}

fn render_sprite(
    mut animations: Query<
        (&AseAnimation, &mut Sprite, &AnimationState),
        // Or<(With<FullyLoadedAnimation>, Changed<AseAnimation>)>,
    >,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (animation, mut target, state) in &mut animations {
        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };
        target.image = aseprite.atlas_image.clone();
        target.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: aseprite.get_atlas_index(usize::from(state.current_frame)),
        });
    }
}

/// Add this tag, if you do not want to plugin to handle
/// anitmation ticks. Instead you can directly control the
/// `AnimationState` component
#[derive(Component)]
pub struct ManualTick;

#[derive(Debug, Clone, Reflect)]
#[reflect]
pub struct Animation {
    pub tag: Option<String>,
    pub speed: f32,
    pub playing: bool,
    pub repeat: AnimationRepeat,
    // overwrite aseprite direction
    pub direction: Option<AnimationDirection>,
    pub queue: VecDeque<(String, AnimationRepeat)>,
    pub hold_relative_frame: bool,
    pub relative_group: u16,
    pub new_relative_group: u16,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            tag: None,
            speed: 1.0,
            playing: false,
            repeat: AnimationRepeat::Loop,
            direction: None,
            queue: VecDeque::new(),
            hold_relative_frame: false,
            relative_group: 0,
            new_relative_group: 0,
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

    /// animation holds relative frame when tag changes, default is false
    pub fn with_relative_frame_hold(mut self, hold_relative_frame: bool) -> Self {
        self.hold_relative_frame = hold_relative_frame;
        self
    }

    /// animation with tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// sets a repeat count, defaults is loop
    pub fn with_repeat(mut self, repeat: AnimationRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    /// provides an animation direction, overwrites aseprite direction
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// chains an animation after the current one is done
    pub fn with_then(mut self, tag: impl Into<String>, repeats: AnimationRepeat) -> Self {
        self.queue.push_back((tag.into(), repeats));
        self
    }

    /// instanly starts playing a new animation, clearing any item left in the queue.
    pub fn play(&mut self, tag: impl Into<String>, repeat: AnimationRepeat) {
        self.tag = Some(tag.into());
        self.repeat = repeat;
        self.queue.clear();
    }

    /// instanly starts playing a new animation starting with same relative frame only if the new relative group is the same as the previous one.
    pub fn play_with_relative_group(
        &mut self,
        tag: impl Into<String>,
        repeat: AnimationRepeat,
        new_relative_group: u16,
    ) {
        self.tag = Some(tag.into());
        self.new_relative_group = new_relative_group;
        self.repeat = repeat;
        self.queue.clear();
    }

    /// instanly starts playing a new animation, clearing any item left in the queue.
    pub fn play_loop(&mut self, tag: impl Into<String>) {
        self.tag = Some(tag.into());
        self.repeat = AnimationRepeat::Loop;
        self.queue.clear();
    }

    /// chains an animation after the current one is done
    pub fn then(&mut self, tag: impl Into<String>, repeats: AnimationRepeat) {
        self.queue.push_back((tag.into(), repeats));
    }

    /// clears any queued up animations
    pub fn clear_queue(&mut self) {
        self.queue.clear()
    }

    fn next(&mut self) {
        if let Some((tag, repeat)) = self.queue.pop_front() {
            self.tag = Some(tag);
            self.repeat = repeat;
        }
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

#[derive(Component, Debug, Default, Reflect)]
#[reflect]
pub struct AnimationState {
    /// carefull, changing the frame out of bounds
    /// may result in panic.
    pub relative_frame: u16,
    pub current_frame: u16,
    pub elapsed: std::time::Duration,
    pub current_direction: PlayDirection,
}

#[allow(unused)]
impl AnimationState {
    pub fn current_frame(&self) -> u16 {
        self.current_frame
    }
    pub fn relative_frame(&self) -> u16 {
        self.relative_frame
    }
}

#[derive(Default, Debug, Reflect)]
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

/// Upadtes all `AseAnimation`s
fn update_aseprite_animation(
    mut cmd: Commands,
    mut animations: Query<(
        Entity,
        &mut AseAnimation,
        &mut AnimationState,
        Has<ManualTick>,
    )>,
    aseprites: Res<Assets<Aseprite>>,
    time: Res<Time>,
) {
    for (entity, mut animation, mut state, is_manual) in &mut animations {
        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };

        let range = match animation.animation.tag.as_ref() {
            Some(tag) => aseprite
                .tags
                .get(tag)
                .map(|meta| meta.range.clone())
                .unwrap(),
            None => 0..=(aseprite.frame_durations.len() as u16 - 1),
        };

        // has to check start and end! because hot reloading can cause
        // animations to be outside of the animation range
        if !range.contains(&state.current_frame) {
            //Default code
            if !animation.animation.hold_relative_frame {
                state.current_frame = *range.start();
                state.relative_frame = 0;
                animation.animation.relative_group = 0;
                animation.animation.new_relative_group = 0;

            // Using relative frame switching
            } else {
                if animation.animation.new_relative_group != animation.animation.relative_group {
                    animation.animation.relative_group = animation.animation.new_relative_group;
                    state.current_frame = *range.start();
                    state.relative_frame = 0;
                    state.elapsed = std::time::Duration::ZERO;
                } else {
                    state.relative_frame =
                        (state.relative_frame) % (*range.end() * range.start() - 1);
                    state.current_frame = *range.start() + state.relative_frame;
                }
            }
        }

        if is_manual {
            return;
        }

        state.elapsed +=
            std::time::Duration::from_secs_f32(time.delta_secs() * animation.animation.speed);

        let Some(frame_duration) = aseprite
            .frame_durations
            .get(usize::from(state.current_frame))
        else {
            return;
        };

        if state.elapsed > *frame_duration {
            cmd.trigger_targets(NextFrameEvent, entity);
            state.elapsed =
                Duration::from_secs_f32(state.elapsed.as_secs_f32() % frame_duration.as_secs_f32());
        }
    }
}

#[derive(Event)]
pub struct NextFrameEvent;

fn next_frame(
    trigger: Trigger<NextFrameEvent>,
    mut events: EventWriter<AnimationEvents>,
    mut animations: Query<(&mut AnimationState, &mut AseAnimation)>,
    aseprites: Res<Assets<Aseprite>>,
) {
    let Ok((mut state, mut ase)) = animations.get_mut(trigger.entity()) else {
        return;
    };

    let Some(aseprite) = aseprites.get(&ase.aseprite) else {
        return;
    };

    let animation = &mut ase.animation;

    let (range, direction) = match animation
        .tag
        .as_ref()
        .map(|t| aseprite.tags.get(t))
        .flatten()
    {
        Some(meta) => {
            let dir = animation
                .direction
                .clone()
                .unwrap_or(AnimationDirection::from(meta.direction));
            (meta.range.clone(), dir)
        }
        None => {
            let dir = animation
                .direction
                .clone()
                .unwrap_or(AnimationDirection::Forward);
            (0..=(aseprite.frame_durations.len() as u16 - 1), dir)
        }
    };

    match direction {
        AnimationDirection::Forward => {
            let next = state.current_frame + 1;

            if next > *range.end() {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_frame = *range.start();
                        state.relative_frame = 0;
                        events.send(AnimationEvents::LoopCycleFinished(trigger.entity()));
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_frame = *range.start();
                            state.relative_frame = 0;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            if animation.queue.is_empty() {
                                events.send(AnimationEvents::Finished(trigger.entity()));
                            } else {
                                animation.next();
                            }
                        }
                    }
                }
            } else {
                state.current_frame = next;
                state.relative_frame += 1;
            }
        }
        AnimationDirection::Reverse => {
            let next = state.current_frame.checked_sub(1).unwrap_or(*range.end());

            if next == *range.end() {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_frame = range.end() - 1;
                        state.relative_frame = range.end() - range.start() - 1;
                        events.send(AnimationEvents::LoopCycleFinished(trigger.entity()));
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_frame = range.end() - 1;
                            state.relative_frame = range.end() - range.start() - 1;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            if animation.queue.is_empty() {
                                events.send(AnimationEvents::Finished(trigger.entity()));
                            } else {
                                animation.next();
                            }
                        }
                    }
                }
            } else {
                state.current_frame = next;
                state
                    .relative_frame
                    .checked_sub(1)
                    .unwrap_or(range.end() - range.start() - 1);
            }
        }
        AnimationDirection::PingPong | AnimationDirection::PingPongReverse => {
            let (next, relative_next) = match state.current_direction {
                PlayDirection::Forward => (state.current_frame + 1, state.relative_frame + 1),
                PlayDirection::Backward => (
                    state.relative_frame.checked_sub(1).unwrap_or(0),
                    state.current_frame.checked_sub(1).unwrap_or(0),
                ),
            };

            let is_forward = match state.current_direction {
                PlayDirection::Forward => true,
                PlayDirection::Backward => false,
            };

            if next >= *range.end() && is_forward {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_direction = PlayDirection::Backward;
                        state.current_frame = range.end() - 2;
                        state.relative_frame = range.end() - range.start() - 2;
                        events.send(AnimationEvents::LoopCycleFinished(trigger.entity()));
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_direction = PlayDirection::Backward;
                            state.current_frame = range.end() - 2;
                            state.relative_frame = range.end() - range.start() - 2;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            if animation.queue.is_empty() {
                                events.send(AnimationEvents::Finished(trigger.entity()));
                            } else {
                                animation.next();
                            }
                        }
                    }
                };
            } else if next <= *range.start() && !is_forward {
                match animation.repeat {
                    AnimationRepeat::Loop => {
                        state.current_direction = PlayDirection::Forward;
                        state.current_frame = *range.start();
                        state.relative_frame = 0;
                        events.send(AnimationEvents::LoopCycleFinished(trigger.entity()));
                    }
                    AnimationRepeat::Count(count) => {
                        if count > 0 {
                            state.current_direction = PlayDirection::Forward;
                            state.current_frame = *range.start();
                            state.relative_frame = 0;
                            animation.repeat = AnimationRepeat::Count(count - 1);
                        } else {
                            if animation.queue.is_empty() {
                                events.send(AnimationEvents::Finished(trigger.entity()));
                            } else {
                                animation.next();
                            }
                        }
                    }
                };
            } else {
                state.current_frame = next;
                state.relative_frame = relative_next;
            }
        }
    };
}

// fn hotreload_animation(
//     mut cmd: Commands,
//     mut events: EventReader<AssetEvent<Aseprite>>,
//     animations: Query<(Entity, &AseAnimation)>,
// ) {
//     for event in events.read() {
//         let AssetEvent::LoadedWithDependencies { id } = event else {
//             continue;
//         };

//         animations
//             .iter()
//             .filter(|(_, animation)| animation.aseprite.id() == *id)
//             .for_each(|(entity, _)| {
//                 cmd.entity(entity).insert(FullyLoadedAnimation);
//             });
//     }
// }

// / component to signal a aseprite render is fully loaded.
// #[derive(Component, Default)]
// pub struct FullyLoadedAnimation;

// pub(crate) fn remove_fully_loaded_animation(
//     mut cmd: Commands,
//     mut nodes: Query<Entity, With<FullyLoadedAnimation>>,
// ) {
//     for entity in nodes.iter_mut() {
//         cmd.entity(entity).remove::<FullyLoadedAnimation>();
//     }
// }
