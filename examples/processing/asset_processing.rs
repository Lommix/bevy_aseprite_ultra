//! This example is identical to the `animation` example, except it uses asset processing.

use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                })
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    // You don't need to do this in your own project, here we are changing the
                    // processed path so that it ends up in the same directory as this file.
                    processed_file_path: "examples/processing/imported_assets/Default".into(),
                    ..Default::default()
                }),
        )
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, events)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("walk-right"),
            aseprite: server.load("player.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(15., 0., 0.)),
    ));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("walk-up"),
            aseprite: server.load("player.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("walk-down"),
            aseprite: server.load("player.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(-15., 0., 0.)),
    ));

    cmd.spawn((
        AseAnimation {
            animation: Animation::default()
                .with_direction(AnimationDirection::Reverse)
                .with_repeat(AnimationRepeat::Count(1)),
            aseprite: server.load("player.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(0., -20., 0.)),
    ));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("walk-right"),
            aseprite: server.load("player.aseprite"),
        },
        Transform::from_translation(Vec3::new(15., -20., 0.)),
        Sprite {
            flip_x: true,
            ..default()
        },
    ));

    cmd.spawn((
        AseAnimation {
            animation: Animation::default().with_tag("squash"),
            aseprite: server.load("ball.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(0., 20., 0.)),
    ));

    cmd.spawn((
        AseSlice {
            name: "ghost_red".into(),
            aseprite: server.load("ghost_slices.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(50., 0., 0.)),
    ));

    cmd.spawn((
        AseSlice {
            name: "ghost_blue".into(),
            aseprite: server.load("ghost_slices.aseprite"),
        },
        Sprite {
            flip_x: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(80., 0., 0.)),
    ));
}

fn events(mut events: EventReader<AnimationEvents>, mut cmd: Commands) {
    for event in events.read() {
        match event {
            AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn(),
            AnimationEvents::LoopCycleFinished(_entity) => (),
        };
    }
}
