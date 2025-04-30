use crate::loader::{Aseprite, SliceMeta};
use bevy::{ecs::component::Mutable, prelude::*, sprite::Anchor};

pub struct AsepriteSlicePlugin;

impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_slice::<Sprite>);
        app.add_systems(Update, render_slice::<Sprite>);
        app.register_type::<AseSlice>();
    }
}

pub trait RenderSlice {
    type Extra;
    fn render_slice(&mut self, aseprite: &Aseprite, slice_meta: &SliceMeta, extra: &Self::Extra);
}

impl RenderSlice for ImageNode {
    type Extra = ();
    fn render_slice(&mut self, aseprite: &Aseprite, slice_meta: &SliceMeta, _extra: &()) {
        self.image = aseprite.atlas_image.clone();
        self.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice_meta.atlas_id,
        });
    }
}

impl RenderSlice for Sprite {
    type Extra = ();
    fn render_slice(&mut self, aseprite: &Aseprite, slice_meta: &SliceMeta, _extra: &()) {
        self.anchor = Anchor::from(slice_meta);
        self.image = aseprite.atlas_image.clone();
        self.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice_meta.atlas_id,
        });
    }
}

/// Displays a aseprite atlas slice
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect]
pub struct AseSlice {
    pub name: String,
    pub aseprite: Handle<Aseprite>,
}

fn render_slice<T: RenderSlice + Component<Mutability = Mutable>>(
    mut nodes: Query<(&mut T, &AseSlice)>,
    aseprites: Res<Assets<Aseprite>>,
    extra: <T as RenderSlice>::Extra,
) {
    for (mut target, slice) in &mut nodes {
        let Some(aseprite) = aseprites.get(&slice.aseprite) else {
            return;
        };
        let Some(slice_meta) = aseprite.slices.get(&slice.name) else {
            warn!("slice does not exists {}", slice.name);
            return;
        };
        target.render_slice(aseprite, slice_meta, &extra);
    }
}
