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
    }
}

#[allow(unused)]
pub mod prelude {
    pub use crate::slice::AsepriteSlice;
    pub use crate::slice::AsepriteSliceBundle;
    pub use crate::BevySprityPlugin;
}
