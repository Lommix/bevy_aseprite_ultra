## 0.5

- new shader example. Render animations to any custom material.
- updated to bevy 0.16

## 0.4.1

- fixed queue system, added example

## 0.4.0

- fixed speed multiplier
- (internal) decoupled next frame logic
- new manual example
- new `NextFrameEvent` to progress animations with custom logic.

## 0.3.3

- new animation now correctly start at the tag start frame.

## 0.3.2

- replaced `basic-universl` with `png` feature.

## 0.3.1

- changing the slice component now updates the sprite/ui.

## 0.3.0

- updated to bevy 0.15
- changed plugin name to `AsepriteUltraPlugin`.
- removed bundles, switched to required components.
- added `ManualTick` component. Let's you update the animation state following you own logic.
- added `FrameChangedEvent`. Triggering it on an entity ensures a frame re-render. (has to be called manual if in manual control mode).
- replaced `anyhow` with `thiserror`.

## 0.2.4

- aseprite slice component can now be changed at runtime.
- increased max size atlas.

## 0.2.3

- non existing animation tags no longer panic, instead default back to play the whole animation file.

---
