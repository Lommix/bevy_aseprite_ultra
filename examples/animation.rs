use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, events)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));

    cmd.spawn((
        AseSpriteAnimation {
            animation: Animation::tag("walk-right"),
            aseprite: server.load("player.aseprite"),
        },
        Transform::from_translation(Vec3::new(15., 0., 0.)),
    ));

    cmd.spawn((
        AseSpriteAnimation {
            animation: Animation::tag("walk-up"),
            aseprite: server.load("player.aseprite"),
        },
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));

    cmd.spawn((
        AseSpriteAnimation {
            animation: Animation::tag("walk-down"),
            aseprite: server.load("player.aseprite"),
        },
        Transform::from_translation(Vec3::new(-15., 0., 0.)),
    ));
    cmd.spawn((
        AseSpriteAnimation {
            animation: Animation::default().with_direction(AnimationDirection::Reverse),
            aseprite: server.load("player.aseprite"),
        },
        Transform::from_translation(Vec3::new(0., -20., 0.)),
    ));

    cmd.spawn((
        AseSpriteAnimation {
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
        AseSpriteAnimation {
            animation: Animation::default().with_tag("squash"),
            aseprite: server.load("ball.aseprite"),
        },
        Transform::from_translation(Vec3::new(0., 20., 0.)),
    ));

    cmd.spawn((
        AseSpriteSlice {
            name: "ghost_red".into(),
            aseprite: server.load("ball.aseprite"),
        },
        Transform::from_translation(Vec3::new(50., 0., 0.)),
    ));

    cmd.spawn((
        AseSpriteSlice {
            name: "ghost_blue".into(),
            aseprite: server.load("ball.aseprite"),
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
            AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn_recursive(),
            AnimationEvents::LoopCycleFinished(_entity) => (),
        };
    }
}
