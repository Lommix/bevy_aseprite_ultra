use crate::{loader::Aseprite, NotLoaded, UiTag};
use bevy::{prelude::*, sprite::Anchor};

pub struct AsepriteSlicePlugin;
impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, insert_aseprite_slice);
        app.register_type::<AsepriteSlice>();
    }
}

/// The `AsepriteSliceBundle` bundles the components needed to render a slice of an aseprite.
/// This is intended to be used for static Sprite Atlases.
/// So only the first frame of your aseprite file will be considered.
///
/// ```rust
/// // example from examples/slices.rs
/// command.spawn(AsepriteSliceBundle {
///    slice: "ghost_red".into(),
///    aseprite: server.load("ghost_slices.aseprite"),
///    sprite: Sprite {
///         flip_x: true,
///         ..default()
///    },
///    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
///     ..default()
/// });
/// ```
#[derive(Bundle, Default)]
pub struct AsepriteSliceBundle {
    pub slice: AsepriteSlice,
    pub aseprite: Handle<Aseprite>,
    pub sprite: Sprite,
    pub atlas: TextureAtlas,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub not_loaded: NotLoaded,
}

/// The `AsepriteSliceUiBundle` bundles the components needed to render a slice of an aseprite in
/// bevy ui. This is intended to be used for static Sprite Atlases.
/// This bundle can be added to any ui node, that contains an `UiImage`
#[derive(Bundle, Default)]
pub struct AsepriteSliceUiBundle {
    pub slice: AsepriteSlice,
    pub aseprite: Handle<Aseprite>,
    pub atlas: TextureAtlas,
    pub not_loaded: NotLoaded,
    pub ui_tag: UiTag,
}

/// The `AsepriteSlice` component is used to specify which slice of an aseprite should be rendered.
/// If the slice is not found in the aseprite file, the game will panic.
#[derive(Component, Default, Reflect)]
pub struct AsepriteSlice(String);

impl AsepriteSlice {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<&str> for AsepriteSlice {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

fn insert_aseprite_slice(
    mut cmd: Commands,
    mut query: Query<
        (
            Entity,
            &mut TextureAtlas,
            &AsepriteSlice,
            &Handle<Aseprite>,
            Option<&UiTag>,
        ),
        With<NotLoaded>,
    >,
    mut sprites: Query<&mut Sprite>,
    aseprites: Res<Assets<Aseprite>>,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut atlas, slice, handle, maybe_ui)| {
            let Some(aseprite) = aseprites.get(handle) else {
                return;
            };

            let slice_meta = aseprite
                .slices
                .get(&slice.0)
                .expect(format!("Slice {} not found in {:?}", slice.0, handle.path()).as_str());

            atlas.layout = aseprite.atlas_layout.clone();
            atlas.index = slice_meta.atlas_id;

            if let Some(mut cmd) = cmd.get_entity(entity) {
                match maybe_ui {
                    Some(_) => {
                        cmd.remove::<NotLoaded>()
                            .insert((UiImage::new(aseprite.atlas_image.clone()),));
                    }
                    None => {
                        if let Ok(mut sprite) = sprites.get_mut(entity) {
                            sprite.anchor = Anchor::from(slice_meta);
                        }
                        cmd.remove::<NotLoaded>()
                            .insert(aseprite.atlas_image.clone());
                    }
                }
            };
        });
}
