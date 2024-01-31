use crate::loader::Aseprite;
use bevy::{prelude::*, sprite::Anchor};

pub struct AsepriteSlicePlugin;
impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, insert_aseprite_slice);
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
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// The `AsepriteSlice` component is used to specify which slice of an aseprite should be rendered.
/// If the slice is not found in the aseprite file, the game will panic.
#[derive(Component, Default)]
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
        (Entity, &mut Sprite, &AsepriteSlice, &Handle<Aseprite>),
        Without<Handle<Image>>,
    >,
    aseprites: Res<Assets<Aseprite>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut sprite, slice, handle)| {
            let Some(aseprite) = aseprites.get(handle) else {
                return;
            };

            let Some(atlas_handle) = &aseprite.atlas else {
                return;
            };

            let Some(atlas) = atlases.get(atlas_handle) else {
                return;
            };

            let slice_meta = aseprite
                .slices
                .get(&slice.0)
                .expect(format!("Slice {} not found in {:?}", slice.0, handle.path()).as_str());

            sprite.rect = Some(slice_meta.rect);
            sprite.anchor = Anchor::from(slice_meta);

            cmd.entity(entity).insert(atlas.texture.clone());
        });
}
