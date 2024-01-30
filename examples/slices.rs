use bevy::prelude::*;
// use bevy_sprity::slice::{AsepriteSlice, AsepriteSliceBundle};
use bevy_sprity::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
                })
                // .set(AssetPlugin {
                //     watch_for_changes_override : Some(true),
                //     ..default()
                // })
                .set(WindowPlugin {
                    primary_window: Some(Window { ..default() }),
                    ..default()
                }),
        )
        .add_plugins(bevy_sprity::BevySprityPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle {
        transform: Transform::default().with_scale(Vec3::splat(0.1)),
        ..default()
    });

    cmd.spawn(AsepriteSliceBundle {
        slice: "ghost_red".into(),
        aseprite: server.load("ghost_slices.aseprite"),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    });

    cmd.spawn(AsepriteSliceBundle {
        slice: AsepriteSlice::new("ghost_blue").flip_x(),
        aseprite: server.load("ghost_slices.aseprite"),
        transform: Transform::from_translation(Vec3::new(32., 0., 0.)),
        ..Default::default()
    });
}
