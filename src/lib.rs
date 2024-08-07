#[allow(unused)]
use bevy::prelude::*;

pub(crate) mod animation;
pub(crate) mod loader;
pub(crate) mod slice;

pub struct BevyAsepriteUltraPlugin {
    pub max_atlas_size: UVec2,
}

impl Default for BevyAsepriteUltraPlugin {
    fn default() -> Self {
        Self {
            max_atlas_size: UVec2::splat(4096),
        }
    }
}

impl Plugin for BevyAsepriteUltraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::AsepriteLoaderPlugin {
            max_atlas_size: self.max_atlas_size,
        });
        app.add_plugins(slice::AsepriteSlicePlugin);
        app.add_plugins(animation::AsepriteAnimationPlugin);
    }
}

pub mod prelude {
    pub use crate::animation::{
        Animation, AnimationDirection, AnimationEvents, AnimationRepeat, AsepriteAnimationBundle,
        AsepriteAnimationUiBundle,
    };
    pub use crate::loader::Aseprite;
    pub use crate::slice::{AsepriteSlice, AsepriteSliceBundle, AsepriteSliceUiBundle};
    pub use crate::BevyAsepriteUltraPlugin;
}

/// tags a bundle as ui node
#[derive(Component, Default)]
pub struct UiTag;

/// tags an entity as not yet loaded;
#[derive(Component, Default)]
pub struct NotLoaded;
