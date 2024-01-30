use crate::loader::{Aseprite, Dirty};
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
    pub transform: Transform,
}

#[derive(Component, Default)]
pub struct AsepriteSlice {
    name: String,
    flip_x: bool,
    flip_y: bool,
}

impl AsepriteSlice {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn flip_x(mut self) -> Self {
        self.flip_x = true;
        self
    }

    pub fn flip_y(mut self) -> Self {
        self.flip_y = true;
        self
    }
}

impl From<&str> for AsepriteSlice {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

fn insert_aseprite_slice(
    mut cmd: Commands,
    query: Query<(Entity, &AsepriteSlice, &Transform, &Handle<Aseprite>), Without<Sprite>>,
    aseprites: Res<Assets<Aseprite>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    query
        .iter()
        .for_each(|(entity, slice, &transform, handle)| {
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
                .get(&slice.name)
                .expect(format!("Slice {} not found in {:?}", slice.name, handle.path()).as_str());

            cmd.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    rect: Some(slice_meta.rect),
                    flip_x: slice.flip_x,
                    flip_y: slice.flip_y,
                    anchor: Anchor::from(slice_meta),
                    ..default()
                },
                texture: atlas.texture.clone(),
                transform,
                ..default()
            });
        });
}
