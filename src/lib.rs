#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub(crate) mod animation;
pub(crate) mod error;
pub(crate) mod loader;
#[cfg(feature = "asset_processing")]
pub(crate) mod processor;
pub(crate) mod slice;

pub mod prelude {
    pub use crate::animation::{
        render_animation, Animation, AnimationDirection, AnimationEvents, AnimationRepeat,
        AnimationState, AseAnimation, ManualTick, NextFrameEvent, PlayDirection, RenderAnimation,
    };
    pub use crate::loader::{Aseprite, AsepriteLoaderSettings, LayerSelectionSetting, SliceMeta};
    pub use crate::slice::{render_slice, AseSlice, NinePatchBehavior, RenderSlice};
    pub use crate::AsepriteUltraPlugin;
}

/// # Aseprite Ultra Plugin
///
/// Quick guide:
///
/// add the plugin to your game
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_aseprite_ultra::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
///         .add_plugins(AsepriteUltraPlugin)
///         .add_systems(Startup, setup)
///         .run();
/// }
///
/// // spawn sprites, animations and ui
/// fn setup(mut cmd: Commands, server: Res<AssetServer>) {
///     // ui animation
///     cmd.spawn((
///         ImageNode::default(),
///         AseAnimation {
///             aseprite: server.load("player.aseprite"),
///             animation: Animation::default().with_tag("walk-right"),
///         },
///     ));
///
///     // sprite animation
///     cmd.spawn((
///         Sprite::default(),
///         AseAnimation {
///             aseprite: server.load("player.aseprite"),
///             animation: Animation::default().with_tag("walk-right"),
///         },
///     ));
///
///     // static sprite slice
///     cmd.spawn((
///         Sprite::default(),
///         AseSlice {
///             name: "ghost_red".into(),
///             aseprite: server.load("ghost_slices.aseprite"),
///         },
///     ));
///
///     // static ui slice
///     cmd.spawn((
///         ImageNode::default(),
///         AseSlice {
///             name: "ghost_red".into(),
///             aseprite: server.load("ghost_slices.aseprite"),
///         },
///     ));
/// }
/// ```
pub struct AsepriteUltraPlugin;
impl Plugin for AsepriteUltraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::AsepriteLoaderPlugin);
        app.add_plugins(slice::AsepriteSlicePlugin);
        app.add_plugins(animation::AsepriteAnimationPlugin);
        #[cfg(feature = "asset_processing")]
        app.add_plugins(processor::AsepriteProcessorPlugin);
    }
}
