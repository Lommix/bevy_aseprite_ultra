use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_aseprite_ultra::prelude::*;

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Walk,
    Stand,
}
#[derive(Debug, PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Component, Debug)]
pub struct Player {
    pub walk_speed: f32,
    pub state: PlayerState,
    pub direction: PlayerDirection,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (control_player, player_animation))
        .run();
}
fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("walk-up")
                .with_repeat(AnimationRepeat::Loop)
                .with_direction(AnimationDirection::Forward)
                .with_speed(2.0),
            aseprite: server.load("player.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(15., 0., 0.)),
        Player {
            walk_speed: 30.,
            state: PlayerState::Stand,
            direction: PlayerDirection::Right,
        },
    ));
}

fn player_animation(mut animation_query: Query<(&mut AseAnimation, &Player)>) {
    for (mut ase_sprite_animation, player) in animation_query.iter_mut() {
        match player.state {
            PlayerState::Stand => {
                ase_sprite_animation.animation.play_loop("idle");
            }
            PlayerState::Walk => match player.direction {
                PlayerDirection::Up => {
                    ase_sprite_animation.animation.play_loop("walk-up");
                }
                PlayerDirection::Down => {
                    ase_sprite_animation.animation.play_loop("walk-down");
                }
                PlayerDirection::Left | PlayerDirection::Right => {
                    ase_sprite_animation.animation.play_loop("walk-right");
                }
            },
        }
    }
}

pub fn control_player(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if query.iter().count() != 1 {
        return;
    }

    for (mut transform, mut player) in query.iter_mut() {
        let mut pressed_flag: bool = false;
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y += player.walk_speed * time.delta_secs();
            player.direction = PlayerDirection::Up;
            pressed_flag = true;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= player.walk_speed * time.delta_secs();
            player.direction = PlayerDirection::Down;
            pressed_flag = true;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= player.walk_speed * time.delta_secs();
            player.direction = PlayerDirection::Left;
            transform.scale.x = -1.;
            pressed_flag = true;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += player.walk_speed * time.delta_secs();
            player.direction = PlayerDirection::Right;
            transform.scale.x = 1.;
            pressed_flag = true;
        }
        if pressed_flag {
            player.state = PlayerState::Walk;
        } else {
            player.state = PlayerState::Stand;
        }
    }
}
