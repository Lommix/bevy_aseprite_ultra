use crate::loader::Aseprite;
use bevy::{prelude::*, sprite::Anchor};

pub struct AsepriteSlicePlugin;

impl Plugin for AsepriteSlicePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     (
        //         // remove_fully_loaded_slice,
        //         // hotreload_slice.run_if(on_event::<AssetEvent<Aseprite>>),
        //     ),
        // );
        app.add_slice_render_system((render_image_node, render_sprite));
        app.register_type::<AseSlice>();
    }
}

pub trait AddSliceRenderSystem {
    fn add_slice_render_system<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self;
}

impl AddSliceRenderSystem for App {
    fn add_slice_render_system<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
        self.add_systems(Update, systems
            // .before(remove_fully_loaded_slice)
            );
        self
    }
}

/// Displays a aseprite atlas slice
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect]
pub struct AseSlice {
    pub name: String,
    pub aseprite: Handle<Aseprite>,
}

fn render_image_node(
    mut nodes: Query<(&mut ImageNode, &AseSlice)
    // , Or<(With<FullyLoadedSlice>, Changed<AseSlice>)>
    >,
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
        target.image = aseprite.atlas_image.clone();
        target.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice_meta.atlas_id,
        });
    }
}

fn render_sprite(
    mut nodes: Query<(&mut Sprite, &AseSlice)
    //, Or<(With<FullyLoadedSlice>, Changed<AseSlice>)>
    >,
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
        target.anchor = Anchor::from(slice_meta);
        target.image = aseprite.atlas_image.clone();
        target.texture_atlas = Some(TextureAtlas {
            layout: aseprite.atlas_layout.clone(),
            index: slice_meta.atlas_id,
        });
    }
}

// fn hotreload_slice(
//     mut cmd: Commands,
//     mut events: EventReader<AssetEvent<Aseprite>>,
//     slices: Query<(Entity, &AseSlice)>,
// ) {
//     for event in events.read() {
//         let AssetEvent::LoadedWithDependencies { id } = event else {
//             continue;
//         };

//         slices
//             .iter()
//             .filter(|(_, slice)| slice.aseprite.id() == *id)
//             .for_each(|(entity, _)| {
//                 cmd.entity(entity).insert(FullyLoadedSlice);
//             });
//     }
// }

///// component to signal a aseprite render is fully loaded.
// #[derive(Component, Default)]
// pub struct FullyLoadedSlice;

// pub(crate) fn remove_fully_loaded_slice(
//     mut cmd: Commands,
//     mut nodes: Query<Entity, With<FullyLoadedSlice>>,
// ) {
//     for entity in nodes.iter_mut() {
//         cmd.entity(entity).remove::<FullyLoadedSlice>();
//     }
// }
