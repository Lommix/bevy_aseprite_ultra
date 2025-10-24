use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, events)
        .run();
}

fn setup<'a>(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("idle")
                .with_repeat(AnimationRepeat::Count(1))
                .with_then("walk-right", AnimationRepeat::Count(1)),

            aseprite: server.load("player.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(15., 0., 0.)),
    ));
}

fn events(mut events: MessageReader<AnimationEvents>, mut cmd: Commands) {
    for event in events.read() {
        match event {
            AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn(),
            AnimationEvents::LoopCycleFinished(_entity) => (),
        };
    }
}
