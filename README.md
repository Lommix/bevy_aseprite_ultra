# Bevy Aseprite Ultra


[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](./LICENSE)
[![Crate](https://img.shields.io/crates/v/bevy_aseprite_ultra.svg)](https://crates.io/crates/bevy_aseprite_ultra)

The ultimate bevy aseprite plugin. This plugin allows you to import aseprite files into bevy, with 100% unbreakable
hot reloading. You can also import static sprites from an aseprite atlas type file using slices with functional pivot offsets!

| Bevy Version | Plugin Version |
| -----------: | -------------: |
|         0.14 |          0.2.0 |
|         0.13 |          0.1.0 |

**The plugin is currently being battle tested and is not released yet**

I use it in my game, check it out on my [blog](https://lommix.com)

## Supported aseprite features

-   Animations
-   Tags
-   Frame duration, repeat, and animation direction
-   Layer visibility
-   Blend modes
-   Static slices and pivot offsets

## Features in bevy

-   Hot reload anything, anytime, anywhere!
-   Full control over animations using Components.
-   One shot animations and events when they finish.
-   Static sprites with slices. Use aseprite for all your icon and ui needs!

(for hotreload to work, you must have the `file_watcher` cargo dependency for bevy installed)

## Example

```bash
cargo run --example slices
cargo run --example animations
```

![Example](docs/example.gif)

<small> character animation by [Benjamin](https://github.com/headcr4sh) </small>

---

There are two main Bundles added by this plugin, `AsepriteAnimationBundle` and `AsepriteSliceBundle`.

```rust
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

...

// Load the an animation from an aseprite file
fn spawn_demo_animation(mut cmd : Commands, server : Res<Assetserver>){
    cmd.spawn(AsepriteAnimationBundle {
        aseprite: server.load("player.aseprite"),
        transform: Transform::from_translation(Vec3::new(15., -20., 0.)),
        animation: Animation::default()
                .with_tag("walk-right")
                .with_speed(2.),
                // Aseprite provides a repeat config per tag, which is beeing ignored on purpose.
                .with_repeat(AnimationRepeat::Count(42)),
                // The direction is provided by the asperite config for the tag, but can be overwritten
                // after the animation is loaded.
                .with_direction(AnimationDirection::PingPong),
                // you can also chain finite animations, loop animations with never finish
                .with_then("walk-left", AnimationRepeat::Count(4))
                .with_then("walk-up", AnimationRepeat::Loop)
        // you can override the default sprite settings here
        sprite: Sprite {
            flip_x: true,
            ..default()
        },
        ..default()
    });
}

// Load a static slice from an aseprite file
fn spawn_demo_static_slice(mut cmd : Commands, server : Res<Assetserver>){
    cmd.spawn(AsepriteSliceBundle {
        slice: "ghost_blue".into(),
        // you can override the default sprite settings here
        // the `rect` will be overriden by the slice
        // if there is a pivot provided in the aseprite slice, the `anchor` will be overwritten
        // and changes the origin of rotation.
        sprite: Sprite {
            flip_x: true,
            ..default()
        },
        aseprite: server.load("ghost_slices.aseprite"),
        transform: Transform::from_translation(Vec3::new(32., 0., 0.)),
        ..default()
    });
}

// animation events - tell me when the animation is done
// this is useful for one shot animations like explosions
fn despawn_on_finish(mut events: EventReader<AnimationEvents>, mut cmd : Commands){
    for event in events.read() {
        match event {
            AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn_recursive(),
            // you can also listen for loop cycle repeats
            AnimationEvents::LoopCycleFinished(_entity) => (),
        };
    }
}
```

## Bevy Ui

There is also an Ui Bundle for Bevy Ui Nodes!

```rust
// animations in bevy ui
cmd.spawn((
        ButtonBundle{
            // node config
            ..default()
        },
        AsepriteAnimationUiBundle{
            aseprite: server.load("yourfile.aseprite"),
            animation: Animation{
                tag : Some("idle".to_string()),
                ..default()
            },
            ..default()
        },
));

// slices in bevy ui
cmd.spawn((
        ImageBundle{
            // node config
            ..default()
        },
        AsepriteSliceUiBundle{
            aseprite: server.load("yourfile.aseprite"),
            slice: AsepriteSlice::from("your_slice"),
            ..default()
        },
));

```
