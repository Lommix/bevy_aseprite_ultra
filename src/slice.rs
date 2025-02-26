use crate::{
    loader::{Aseprite, SliceMeta},
    FullyLoaded,
};
use bevy::{prelude::*, sprite::Anchor};

pub struct AsepriteSlicePlugin;
impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_aseprite_slice,
                render_aseprite_slice::<Sprite>.after(update_aseprite_slice),
                render_aseprite_slice::<ImageNode>.after(update_aseprite_slice),
                hotreload_slice.run_if(on_event::<AssetEvent<Aseprite>>),
            ),
        );
        app.register_type::<AseSlice>();
    }
}

/// Displays a aseprite atlas slice
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect]
pub struct AseSlice {
    pub name: String,
    pub aseprite: Handle<Aseprite>,
}

pub trait AseSliceRender: Component {
    fn render(&mut self, slice_meta: &SliceMeta, aseprite: &Aseprite);
}

impl AseSliceRender for ImageNode {
    fn render(&mut self, slice_meta: &SliceMeta, aseprite: &Aseprite) {
        self.image = aseprite.atlas_image.clone();
        self.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice_meta.atlas_id,
        });
    }
}

impl AseSliceRender for Sprite {
    fn render(&mut self, slice: &SliceMeta, aseprite: &Aseprite) {
        self.anchor = Anchor::from(slice);
        self.image = aseprite.atlas_image.clone();
        self.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice.atlas_id,
        });
    }
}

/// Upadtes all `AseSlice`s
fn update_aseprite_slice(
    mut cmd: Commands,
    mut nodes: Query<Entity, Or<((With<AseSlice>, Without<FullyLoaded>), Changed<AseSlice>)>>,
) {
    for entity in nodes.iter_mut() {
        cmd.entity(entity).insert(FullyLoaded);
    }
}

/// Renders all `AseSlice`s to any `Component` that implements `AseSliceRender`
/// Implement AseAnimationRender for your own custom targets
/// Or create your own render function as seen in the alternative_target example
fn render_aseprite_slice<T: AseSliceRender>(
    mut nodes: Query<(&mut T, &AseSlice), Or<(Added<FullyLoaded>, Changed<AseSlice>)>>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (mut target, slice) in nodes.iter_mut() {
        let Some(aseprite) = aseprites.get(&slice.aseprite) else {
            return;
        };
        let Some(slice_meta) = aseprite.slices.get(&slice.name) else {
            warn!("slice does not exists {}", slice.name);
            return;
        };
        target.render(slice_meta, aseprite);
    }
}

fn hotreload_slice(
    mut cmd: Commands,
    mut events: EventReader<AssetEvent<Aseprite>>,
    slices: Query<(Entity, &AseSlice)>,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };

        slices
            .iter()
            .filter(|(_, slice)| slice.aseprite.id() == *id)
            .for_each(|(entity, _)| {
                cmd.entity(entity).remove::<FullyLoaded>();
            });
    }
}
