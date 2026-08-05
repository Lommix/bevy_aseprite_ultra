use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.1))));

    cmd.spawn(Node {
        width: percent(100),
        height: percent(100),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    })
    .with_children(|parent| {
        for [x, y, w, h] in [
            [10.0, 10.0, 150.0, 150.0],
            [200.0, 10.0, 300.0, 150.0],
            [700.0, 10.0, 150.0, 300.0],
        ] {
            parent
                .spawn((
                    AseSlice {
                        name: "border".into(),
                        aseprite: server.load("nine_patch.aseprite"),
                    },
                    NineSliceBehavior::Enabled,
                    ImageNode::default(),
                    Node {
                        width: px(w),
                        height: px(h),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(px(20)),
                        ..Default::default()
                    },
                    Transform::default().with_translation(Vec3::new(x, y, 0.0)),
                ))
                .with_child((
                    Text::new("Button"),
                    TextFont {
                        font_size: FontSize::Px(33.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        }
    });
}
