use crate::loader::Aseprite;
use bevy::{prelude::*, sprite::Anchor};

pub struct AsepriteSlicePlugin;
impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, insert_aseprite_slice);
    }
}

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
