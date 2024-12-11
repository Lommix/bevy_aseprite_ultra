# Bevy Aseprite Ultra

[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](./LICENSE)
[![Crate](https://img.shields.io/crates/v/bevy_aseprite_ultra.svg)](https://crates.io/crates/bevy_aseprite_ultra)

The ultimate bevy aseprite plugin. This plugin allows you to import aseprite files into bevy, with 100% unbreakable
hot reloading. You can also import static sprites from an aseprite atlas type file using slices with functional pivot offsets!

| Bevy Version | Plugin Version |
| -----------: | -------------: |
|         0.15 |          0.4.0 |
|         0.14 |          0.2.4 |
|         0.13 |          0.1.0 |

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
-   Static sprites with slices. Use aseprite for all your icon and UI needs!

(hot reloading requires the `file_watcher` feature in bevy)

## Embedding Assets

There is currently no asset preprocessor. If you do not want to ship raw aseprite files, use [`bevy_embedded_assets`](https://github.com/vleue/bevy_embedded_assets)
to embed your assets into the final binary.

## Example

```bash
cargo run --example slices
cargo run --example animations
cargo run --example ui
```

![Example](docs/example.gif)

<small> character animation by [Benjamin](https://github.com/headcr4sh) </small>

---

```rust
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

...

// Load an animation from an aseprite file
fn spawn_demo_animation(mut cmd : Commands, server : Res<Assetserver>){
    cmd.spawn((
        AseSpriteAnimation {
            aseprite: server.load("player.aseprite"),
            animation: Animation::tag("walk-right")
                .with_speed(2.)
                // Aseprite provides a repeat config per tag, which is beeing ignored on purpose.
                .with_repeat(AnimationRepeat::Count(42))
                // The direction is provided by the asperite config for the tag, but can be overwritten.
                .with_direction(AnimationDirection::PingPong)
                // you can also chain finite animations, loop animations will never finish
                .with_then("walk-left", AnimationRepeat::Count(4))
                .with_then("walk-up", AnimationRepeat::Loop),
        },
        Sprite {
            // under the hood its just sprites.
            // only the image and atlas is touched.
            // this works
            flip_x: true,
            ..default()
        },
    ));
}

// Load a static slice from an aseprite file
// create for any static atlas with marked regions aka slices.
fn spawn_demo_static_slice(mut cmd : Commands, server : Res<Assetserver>){
    cmd.spawn((
        AseSpriteSlice {
            name: "ghost_red".into(),
            aseprite: server.load("ball.aseprite"),
        },
    ));
}

// animation events
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
        Button,
        AseUiAnimation {
            aseprite: server.load("player.aseprite"),
            animation: Animation::tag("walk-right"),
        },
));

// slices in bevy ui
cmd.spawn((
        Node {
            width: Val::Px(100.),
            height: Val::Px(100.),
            border: UiRect::all(Val::Px(5.)),
            ..default()
        },
        AseUiSlice {
            name: "ghost_red".into(),
            aseprite: server.load("ghost_slices.aseprite"),
        },
));
```
