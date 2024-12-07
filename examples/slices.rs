use std::time::Duration;

use bevy::{image::ImageSamplerDescriptor, prelude::*, time::common_conditions::on_timer};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            change_slice.run_if(on_timer(Duration::from_secs(2))),
        )
        .run();
}

#[derive(Component)]
pub struct SliceCycle {
    current: usize,
    slices: Vec<String>,
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.1))));

    cmd.spawn((
        AseSpriteSlice {
            name: "ghost_red".into(),
            aseprite: server.load("ghost_slices.aseprite"),
        },
        Transform::from_translation(Vec3::new(0., 0., 0.))
            .with_rotation(Quat::from_rotation_z(0.2)),
        SliceCycle {
            current: 0,
            slices: vec!["ghost_red".into(), "ghost_blue".into()],
        },
    ));

    cmd.spawn((
        AseSpriteSlice {
            name: "ghost_blue".into(),
            aseprite: server.load("ghost_slices.aseprite"),
        },
        Sprite {
            flip_x: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(32., 0., 0.)),
    ));
}

fn change_slice(mut slices: Query<(&mut AseSpriteSlice, &mut SliceCycle)>) {
    slices.iter_mut().for_each(|(mut slice, mut cycle)| {
        cycle.current += 1;
        let index = cycle.current % cycle.slices.len();
        slice.name = cycle.slices[index].clone();
        info!("slice changed to {}", slice.name);
    });
}
