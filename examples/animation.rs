use bevy::prelude::*;
use bevy_sprity::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(bevy_sprity::BevySprityPlugin)
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
        transform: Transform::from_translation(Vec3::new(15., 0., 0.)),
        ..default()
    })
    .insert(AnimationTag::from("walk-right"));

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    })
    .insert(AnimationTag::from("walk-up"));

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        transform: Transform::from_translation(Vec3::new(-15., 0., 0.)),
        ..default()
    })
    .insert(AnimationTag::from("walk-down"));

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        animation_speed: AnimationSpeed(3.0),
        transform: Transform::from_translation(Vec3::new(0., -20., 0.)),
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        transform: Transform::from_translation(Vec3::new(15., -20., 0.)),
        animation_speed: AnimationSpeed(0.5),
        sprite: Sprite {
            flip_x: true,
            ..default()
        },
        ..default()
    })
    .insert(AnimationTag::from("walk-right"));

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("ball.aseprite"),
        animation_speed: AnimationSpeed(2.0),
        transform: Transform::from_translation(Vec3::new(0., 20., 0.)),
        ..default()
    })
    .insert(AnimationTag::from("squash"));



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


fn events(mut events: EventReader<AnimationEvents>, mut cmd: Commands) {
    for event in events.read() {
        match event {
            AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn_recursive(),
            AnimationEvents::LoopCycleFinished(_entity) => (),
        };
    }
}
