use bevy::{color::palettes::css, prelude::*};
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
            AseUiAnimation {
                aseprite: server.load("player.aseprite").into(),
                animation: Animation::default().with_tag("walk-right"),
            },
        ))
        .id();

    cmd.entity(div).add_child(animation);
}
