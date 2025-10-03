use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_frame)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("walk-right"),
            aseprite: server.load("player.aseprite"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(15., 0., 0.)),
        ManualTick,
    ));

    cmd.spawn(Text("Click for tick".into()));
}

fn update_frame(
    mut cmd: Commands,
    animation_entity: Single<Entity, With<AseAnimation>>,
    inputs: Res<ButtonInput<MouseButton>>,
) {
    if inputs.just_pressed(MouseButton::Left) {
        cmd.trigger(NextFrameEvent(*animation_entity));
    }
}
