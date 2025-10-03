use crate::loader::{Aseprite, SliceMeta};
use bevy::{
    ecs::component::Mutable, prelude::*, sprite::Anchor, sprite_render::Material2d, ui::UiSystems,
};

pub struct AsepriteSlicePlugin;

impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            render_slice::<ImageNode>.before(UiSystems::Prepare),
        );
        app.add_systems(PostUpdate, render_slice::<Sprite>);
        app.register_type::<AseSlice>();
    }
}

/// Anything component that implements this trait is a render target for [`AseSlice`]
///
/// # Examples
/// ```
/// impl RenderSlice for MyMaterial {
///     type Extra<'e> = Res<'e, Time>;
///     fn render_slice(
///         &mut self,
///         aseprite: &Aseprite,
///         slice_meta: &SliceMeta,
///         extra: &mut Self::Extra<'_>,
///     ) {
///         self.image = aseprite.atlas_image.clone();
///         self.texture_min = slice_meta.rect.min.as_uvec2();
///         self.texture_max = slice_meta.rect.max.as_uvec2();
///         self.time = extra.elapsed_secs();
///     }
/// }
/// ```
pub trait RenderSlice {
    /// An extra system parameter used in rendering. Use a tuple if many are required.
    type Extra<'e>;
    fn render_slice(
        &mut self,
        aseprite: &Aseprite,
        slice_meta: &SliceMeta,
        extra: &mut Self::Extra<'_>,
    );
}

impl RenderSlice for ImageNode {
    type Extra<'e> = ();
    fn render_slice(&mut self, aseprite: &Aseprite, slice_meta: &SliceMeta, _extra: &mut ()) {
        self.image = aseprite.atlas_image.clone();
        self.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice_meta.atlas_id,
        });
    }
}

impl RenderSlice for Sprite {
    type Extra<'e> = ();
    fn render_slice(&mut self, aseprite: &Aseprite, slice_meta: &SliceMeta, _extra: &mut ()) {
        self.image = aseprite.atlas_image.clone();
        self.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice_meta.atlas_id,
        });
    }
}

impl<M: Material2d + RenderSlice> RenderSlice for MeshMaterial2d<M> {
    type Extra<'e> = (ResMut<'e, Assets<M>>, <M as RenderSlice>::Extra<'e>);
    fn render_slice(
        &mut self,
        aseprite: &Aseprite,
        slice_meta: &SliceMeta,
        extra: &mut Self::Extra<'_>,
    ) {
        let Some(material) = extra.0.get_mut(&*self) else {
            return;
        };
        material.render_slice(aseprite, slice_meta, &mut extra.1);
    }
}

#[cfg(feature = "3d")]
impl<M: Material + RenderSlice> RenderSlice for MeshMaterial3d<M> {
    type Extra<'e> = (ResMut<'e, Assets<M>>, <M as RenderSlice>::Extra<'e>);
    fn render_slice(
        &mut self,
        aseprite: &Aseprite,
        slice_meta: &SliceMeta,
        extra: &mut Self::Extra<'_>,
    ) {
        let Some(material) = extra.0.get_mut(&*self) else {
            return;
        };
        material.render_slice(aseprite, slice_meta, &mut extra.1);
    }
}

/// Displays a aseprite atlas slice
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect]
pub struct AseSlice {
    pub name: String,
    pub aseprite: Handle<Aseprite>,
}

pub fn render_slice<T: RenderSlice + Component<Mutability = Mutable>>(
    mut slices: Query<(&mut T, Ref<AseSlice>, Option<&mut Anchor>)>,
    aseprites: Res<Assets<Aseprite>>,
    mut extra: <T as RenderSlice>::Extra<'_>,
) {
    let asset_change = aseprites.is_changed();

    for (mut target, slice, maybe_anchor) in &mut slices {
        if !asset_change && !slice.is_changed() {
            continue;
        }
        let Some(aseprite) = aseprites.get(&slice.aseprite) else {
            continue;
        };
        let Some(slice_meta) = aseprite.slices.get(&slice.name) else {
            warn!("slice does not exist {}", slice.name);
            continue;
        };

        if let Some(mut anchor) = maybe_anchor {
            *anchor = Anchor::from(slice_meta);
        }

        target.render_slice(aseprite, slice_meta, &mut extra);
    }
}
