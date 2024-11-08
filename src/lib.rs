#[allow(unused)]
use bevy::prelude::*;

pub(crate) mod animation;
pub(crate) mod error;
pub(crate) mod loader;
pub(crate) mod slice;

pub mod prelude {
    pub use crate::animation::{
        Animation, AnimationDirection, AnimationEvents, AnimationRepeat, AnimationState,
        AseSpriteAnimation, AseUiAnimation, FrameChangedEvent, PlayDirection,
    };
    pub use crate::loader::Aseprite;
    pub use crate::slice::{AseSpriteSlice, AseUiSlice};
    pub use crate::AsepriteUltraPlugin;
}

/// # Aseprite Ultra Plugin
///
/// Quick guide:
///
/// add to game
/// ```rust
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins.set(ImagePlugin {
///             default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
///         }))
///         .add_plugins(AsepriteUltraPlugin)
///         .add_systems(Startup, setup)
///         .run();
/// }
///
/// // spawn sprites, animations and ui
/// fn setup(mut cmd: Commands, server: Res<AssetServer>) {
///     // ui animation
///     cmd.spawn(AseUiAnimation {
///         aseprite: server.load("player.aseprite").into(),
///         animation: Animation::default().with_tag("walk-right"),
///     });
///
///     // sprite animation
///     cmd.spawn(AseSpriteAnimation {
///         aseprite: server.load("player.aseprite").into(),
///         animation: Animation::default().with_tag("walk-right"),
///     });
///
///     // static sprite
///     cmd.spawn(AseSpriteSlice {
///         name: "ghost_red".into(),
///         aseprite: server.load("ghost_slices.aseprite"),
///     });
///
///     // static ui
///     cmd.spawn(AseUiSlice {
///         name: "ghost_red".into(),
///         aseprite: server.load("ghost_slices.aseprite"),
///     });
/// }
/// ```
pub struct AsepriteUltraPlugin;
impl Plugin for AsepriteUltraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::AsepriteLoaderPlugin);
        app.add_plugins(slice::AsepriteSlicePlugin);
        app.add_plugins(animation::AsepriteAnimationPlugin);
    }
}

/// component to signal a aseprite render is fully loaded.
#[derive(Component, Default)]
pub(crate) struct FullyLoaded;
