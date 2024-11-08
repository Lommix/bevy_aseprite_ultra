use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.1))));

    cmd.spawn((
        AseSpriteSlice {
            name: "ghost_red".into(),
            aseprite: server.load("ball.aseprite"),
        },
        Transform::from_translation(Vec3::new(0., 0., 0.))
            .with_rotation(Quat::from_rotation_z(0.2)),
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
        Transform::from_translation(Vec3::new(32., 0., 0.)),
    ));
}
