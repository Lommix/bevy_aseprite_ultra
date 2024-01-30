#[allow(unused)]
use bevy::prelude::*;

pub(crate) mod animation;
pub(crate) mod loader;
pub(crate) mod slice;

pub struct BevySprityPlugin;
impl Plugin for BevySprityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::AsepriteLoaderPlugin);
        app.add_plugins(slice::AsepriteSlicePlugin);
        app.add_plugins(animation::AsepriteAnimationPlugin);
    }
}

#[allow(unused)]
pub mod prelude {
    pub use crate::animation::{
        AnimationDirection, AnimationFrame, AnimationRepeat, AnimationTag, AsepriteAnimationBundle,
    };
    pub use crate::slice::{AsepriteSlice, AsepriteSliceBundle};
    pub use crate::BevySprityPlugin;
}
