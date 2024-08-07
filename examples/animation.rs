use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(BevyAsepriteUltraPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, events)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle {
        transform: Transform::default().with_scale(Vec3::splat(0.15)),
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        animation: Animation::default().with_tag("walk-right"),
        transform: Transform::from_translation(Vec3::new(15., 0., 0.)),
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        animation: Animation::default().with_tag("walk-up").with_speed(0.5),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        animation: Animation::default().with_tag("walk-down"),
        transform: Transform::from_translation(Vec3::new(-15., 0., 0.)),
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        transform: Transform::from_translation(Vec3::new(0., -20., 0.)),
        animation: Animation::default().with_direction(AnimationDirection::Reverse),
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        transform: Transform::from_translation(Vec3::new(15., -20., 0.)),
        animation: Animation::default().with_tag("walk-right"),
        sprite: Sprite {
            flip_x: true,
            ..default()
        },
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("ball.aseprite"),
        animation: Animation::default().with_tag("squash"),
        transform: Transform::from_translation(Vec3::new(0., 20., 0.)),
        ..default()
    });

    cmd.spawn(AsepriteSliceBundle {
        slice: "ghost_red".into(),
        aseprite: server.load("ghost_slices.aseprite"),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..Default::default()
    });

    cmd.spawn(AsepriteSliceBundle {
        slice: "ghost_blue".into(),
        sprite: Sprite {
            flip_x: true,
            ..default()
        },
        aseprite: server.load("ghost_slices.aseprite"),
        transform: Transform::from_translation(Vec3::new(80., 0., 0.)),
        ..Default::default()
    });
}

fn events(mut events: EventReader<AnimationEvents>, mut cmd: Commands) {
    for event in events.read() {
        match event {
            AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn_recursive(),
            AnimationEvents::LoopCycleFinished(_entity) => (),
        };
    }
}
