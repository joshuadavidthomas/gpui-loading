# gpui-loading

Beautiful loading indicators for [GPUI](https://gpui.rs) — a Rust port of
[loading-dev](https://github.com/jakubkrehel/loading) by Jakub Krehel.

The spinners are built on Zed's GPUI, from its `gpui-pre` snapshot on
crates.io:

```toml
gpui = { package = "gpui-pre", version = "=0.3.7" }
```

Spinners draw some shapes from SVGs, which GPUI loads through the app's
asset source, so install theirs when creating the app:

```rust
gpui_platform::application().with_assets(gpui_loading::SpinnerAssets::new())
// or, if the app has assets of its own:
gpui_platform::application().with_assets(gpui_loading::SpinnerAssets::wrap(MyAssets))
```

Then:

```rust
use gpui_loading::{Arc, Cap, Easing};

// Inside a render method:
Arc::new("saving").size(px(16.))
Arc::new("upload").easing(Easing::EaseInOut).cap(Cap::Flat).color(rgb(0x3b82f6))
```

Every spinner takes `size`, `color`, `duration` and `play_state`, and
implements `Styled`, so margins, opacity and the like work as on a `div`.
Some also take `easing`, `cap`, or an option of their own (`Blocks::sweep`,
`Ripple::direction`, `Wave::origin`). `color` defaults to the inherited
text colour, the way the web version paints with `currentColor`.

Each spinner needs an id, unique among its siblings, as any stateful GPUI
element does: it keys the spinner's animation clock, so pausing freezes it on
the current frame and resuming continues from there.

To pick a spinner by name, use `AnySpinner::named(SpinnerName::Radar, id)`;
`SpinnerName::ALL` lists them all with their default durations.
`AnySpinner` also accepts `easing`, `cap`, `sweep`, `direction`, and `origin`;
each option applies to the spinners that support it.

## Reduced motion

GPUI does not expose the platform's reduced-motion setting, so the app sets
it: `set_reduced_motion(cx, ReducedMotion::Reduce)` makes every spinner show the
same still frame the web version shows under `prefers-reduced-motion`.

## Spinners

`Arc`, `Atom`, `Blocks`, `BouncingDots`, `Cascade`, `CircularDots`, `Classic`,
`ClassicV2`, `Clock`, `Comet`, `Compass`, `Dual`, `Eclipse`, `Flip`, `Gather`,
`Leap`, `LinearDots`, `Loading`, `Morph`, `Orbit`, `Pulse`, `Radar`, `Ring`,
`Ripple`, `Slide`, `Snake`, `Swirl`, `Trace`, `Wave`.

```sh
cargo run -p gallery
```

The gallery exposes every spinner’s options, with upstream choice labels,
defaults, and speed ranges. Its live preview and copied Rust example follow
the selected settings; Reset restores them all.

## Rendering

Spinners paint only the primitives GPUI draws cheaply. Dots, bars, squares
and rings are quads. Everything else is an SVG that GPUI rasterizes once into
its sprite atlas, then turns and tints on the GPU each frame, the way Zed
spins its own icons. Nothing is drawn as a path: every batch of paths costs
GPUI a full-window multisampled pass.

Glyphs and gradients that only turn are SVG files in `assets/`. Arcs, whose
length and end caps vary, are generated, as are the shapes that change as
they move, like Snake's stretching dash: one SVG per step, each rendered
once.

## Differences from the web version

- Motion props do not cascade. On the web, `--ld-duration` and
  `--ld-play-state` set on an ancestor reach every spinner below it; here each
  spinner takes its props directly.
- Snake's dash, Trace's offset, Morph's radius, Gather's pull, Atom's tumble
  and Flip's turn move in small steps (a quarter unit, a degree and so on)
  rather than continuously, so that the sprites they need can be cached.

## Development

Tools are pinned in `mise.toml` (`mise install`); recipes are in the
`Justfile`:

- `just test`, `just clippy`, `just fmt` (on the nightly pinned in
  `tools/rustfmt/`), `just hawk` (on the toolchain in `tools/hawk/`)
- `just lint` runs every pre-commit hook with `prek`
- `just gallery` opens the gallery

## License

MIT, as the original.
