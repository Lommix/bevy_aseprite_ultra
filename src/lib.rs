#[allow(unused)]
use bevy::prelude::*;

pub(crate) mod animation;
pub(crate) mod error;
pub(crate) mod loader;
pub(crate) mod slice;

pub mod prelude {
    pub use crate::animation::{
        Animation, AnimationDirection, AnimationEvents, AnimationRepeat, AseSpriteAnimation,
        AseUiAnimation, PlayDirection,
    };
    pub use crate::loader::Aseprite;
    pub use crate::slice::{AseSpriteSlice, AseUiSlice};
    pub use crate::AsepriteUltraPlugin;
}

/// Aseprite Ultra Plugin
///
///
pub struct AsepriteUltraPlugin;
impl Plugin for AsepriteUltraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::AsepriteLoaderPlugin);
        app.add_plugins(slice::AsepriteSlicePlugin);
        app.add_plugins(animation::AsepriteAnimationPlugin);
    }
}

/// tag component to ensure,
#[derive(Component, Default)]
pub(crate) struct FullyLoaded;
