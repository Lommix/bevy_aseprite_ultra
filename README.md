# Bevy Sprity

The ultimate bevy Aseprite plugin. This plugin allows you to import aseprite files into bevy, with 100% unbreakable
hot reloading. You can also import static sprites from an aseprite atlas using slices with functional pivot offsets!

# Supported Aseprite Features

-   Animations
-   Tags
-   Frame Duration, Repeat, and Animation Direction
-   Layer Visibility
-   Blend Modes
-   Static Slices and Pivot offsets

# Features

-   Hotreload anything, anytime, anywhere!
-   Full control over animations using Components.
-   Oneshot animations and events when they finish.

# Example

There are two main Bundles added by this plugin, `AsepriteAnimationBundle` and `AsepriteSliceBundle`.

```rust

// Load the aseprite file
pub struct AsepriteAnimationBundle {
    pub aseprite: Handle<Aseprite>,
    pub animation_frame: AnimationFrame,
    pub animation_speed: AnimationSpeed,
    pub animation_repeat: AnimationRepeat,
    pub animation_direction: AnimationDirection,
    pub animation_state: AnimationState,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

pub struct AsepriteSliceBundle {
    pub slice: AsepriteSlice,
    pub aseprite: Handle<Aseprite>,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

```
