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

Stationary dots, bars, squares and rings are quads. Moving or scaling shapes
are cached SVG sprites, transformed and tinted continuously on the GPU.
GPUI snaps quad bounds and border widths to device pixels, so animated
geometry must not use quads.

Glyphs, gradients, rings and filled circles are SVG files in `assets/`.
Other fixed shapes are generated once and reused. Shapes whose geometry
changes, like Snake's stretching dash, use a bounded set of generated sprites.

Wave changes its bars' height while keeping their ends circular. Its five
bars are batched into one vector path, preserving fractional edges without
distorting the ends or growing the sprite cache. This requires GPUI's
multisampled path pass; the other spinners use quads and sprites.

All 29 spinners have been checked for animated quad bounds:

| Rendering | Spinners |
| --- | --- |
| Continuous sprite transforms | Arc, Atom, Blocks, BouncingDots, Cascade, Clock, Comet, Compass, Dual, Eclipse, Gather, Leap, Orbit, Pulse, Radar, Ring, Ripple, Slide |
| Stationary geometry, opacity animation | CircularDots, Classic, ClassicV2, LinearDots, Loading, Swirl |
| Continuous vector geometry | Wave |
| Cached shape steps, continuous transforms where applicable | Flip, Morph, Snake, Trace |

## Differences from the web version

- Motion props do not cascade. On the web, `--ld-duration` and
  `--ld-play-state` set on an ancestor reach every spinner below it; here each
  spinner takes its props directly.
- Snake's dash, Trace's offset, Morph's radius and Flip's turn move in small
  steps (a quarter unit, a degree and so on)
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
