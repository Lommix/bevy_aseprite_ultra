use bevy::prelude::*;
use bevy_sprity::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(bevy_sprity::BevySprityPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle {
        transform: Transform::default().with_scale(Vec3::splat(0.1)),
        ..default()
    });

    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        animation: "walk-right".into(),
    });
}
