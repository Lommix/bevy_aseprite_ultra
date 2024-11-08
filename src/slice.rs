use crate::{loader::Aseprite, FullyLoaded};
use bevy::{prelude::*, sprite::Anchor};

pub struct AsepriteSlicePlugin;
impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_aseprite_sprite_slice,
                update_aseprite_ui_slice,
                hotreload_ui_slice.run_if(on_event::<AssetEvent<Aseprite>>),
            ),
        );
        app.register_type::<AseSpriteSlice>();
    }
}

/// Displays a aseprite atlas slice
/// on an UI entity.
#[derive(Component, Reflect, Default, Debug, Clone)]
#[require(UiImage)]
#[reflect]
pub struct AseUiSlice {
    pub name: String,
    pub aseprite: Handle<Aseprite>,
}

/// Displays a aseprite atlas slice
/// on a sprite entity.
#[derive(Component, Reflect, Default, Debug, Clone)]
#[require(Sprite)]
#[reflect]
pub struct AseSpriteSlice {
    pub name: String,
    pub aseprite: Handle<Aseprite>,
}

fn update_aseprite_ui_slice(
    mut cmd: Commands,
    mut nodes: Query<(Entity, &mut UiImage, &AseUiSlice), Without<FullyLoaded>>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (entity, mut image, slice) in nodes.iter_mut() {
        let Some(aseprite) = aseprites.get(&slice.aseprite) else {
            continue;
        };

        let Some(slice) = aseprite.slices.get(&slice.name) else {
            warn!("slice does not extists {}", slice.name);
            continue;
        };

        image.image = aseprite.atlas_image.clone();
        image.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice.atlas_id,
        });

        cmd.entity(entity).insert(FullyLoaded);
    }
}

fn update_aseprite_sprite_slice(
    mut cmd: Commands,
    mut nodes: Query<(Entity, &mut Sprite, &AseUiSlice), Without<FullyLoaded>>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (entity, mut sprite, slice) in nodes.iter_mut() {
        let Some(aseprite) = aseprites.get(&slice.aseprite) else {
            continue;
        };

        let Some(slice) = aseprite.slices.get(&slice.name) else {
            warn!("slice does not extists {}", slice.name);
            continue;
        };

        sprite.anchor = Anchor::from(slice);
        sprite.image = aseprite.atlas_image.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice.atlas_id,
        });

        cmd.entity(entity).insert(FullyLoaded);
    }
}

fn hotreload_ui_slice(
    mut cmd: Commands,
    mut events: EventReader<AssetEvent<Aseprite>>,
    ui_slices: Query<(Entity, &AseUiSlice), With<FullyLoaded>>,
    sprite_slices: Query<(Entity, &AseSpriteSlice), With<FullyLoaded>>,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };

        ui_slices
            .iter()
            .filter(|(_, slice)| slice.aseprite.id() == *id)
            .for_each(|(entity, _)| {
                cmd.entity(entity).remove::<FullyLoaded>();
            });

        sprite_slices
            .iter()
            .filter(|(_, slice)| slice.aseprite.id() == *id)
            .for_each(|(entity, _)| {
                cmd.entity(entity).remove::<FullyLoaded>();
            });
    }
}
