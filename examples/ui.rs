use std::time::Duration;

use bevy::{
    color::palettes::css, image::ImageSamplerDescriptor, prelude::*,
    time::common_conditions::on_timer,
};
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

    let div = cmd
        .spawn((
            Node {
                width: Val::Px(300.),
                height: Val::Px(200.),
                padding: UiRect::all(Val::Px(10.)),
                border: UiRect::all(Val::Px(5.)),
                ..default()
            },
            BorderColor(css::BLUE.into()),
            BorderRadius::all(Val::Px(15.)),
        ))
        .id();

    let img = cmd
        .spawn((
            Node {
                width: Val::Px(100.),
                height: Val::Px(100.),
                border: UiRect::all(Val::Px(5.)),
                ..default()
            },
            AseUiSlice {
                name: "ghost_red".into(),
                aseprite: server.load("ghost_slices.aseprite"),
            },
            SliceCycle {
                current: 0,
                slices: vec!["ghost_red".into(), "ghost_blue".into()],
            },
        ))
        .id();

    cmd.entity(div).add_child(img);

    let animation = cmd
        .spawn((
            Node {
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            AseAnimation {
                aseprite: server.load("player.aseprite").into(),
                animation: Animation::default().with_tag("walk-right"),
            },
            ImageNode::default(),
        ))
        .id();

    cmd.entity(div).add_child(animation);
}

fn change_slice(mut slices: Query<(&mut AseUiSlice, &mut SliceCycle)>) {
    slices.iter_mut().for_each(|(mut slice, mut cycle)| {
        cycle.current += 1;
        let index = cycle.current % cycle.slices.len();
        slice.name = cycle.slices[index].clone();
        info!("slice changed to {}", slice.name);
    });
}
