## 3.0.0

-   updated to bevy 0.15
-   changed plugin name to `AsepriteUltraPlugin`.
-   removed bundles, switched to required components.
-   added `ManualTick` component. Let's you update the animation state following you own logic.
-   added `FrameChangedEvent`. Triggering it on an entity ensures a frame re-render. (has to be called manual if in manual control mode).
-   replaced `anyhow` with `thiserror`.

## 2.2.4

-   aseprite slice component can now be changed at runtime.
-   increased max size atlas.

## 2.2.3

-   non existing animation tags no longer panic, instead default back to play the whole animation file.

---
