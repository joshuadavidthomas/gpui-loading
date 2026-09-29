//! Shapes GPUI cannot draw as quads — arcs, glyphs, gradients — as SVGs
//! GPUI rasterizes once into its sprite atlas and then only turns and tints
//! on the GPU each frame, the way Zed spins its own icons.
//!
//! Shapes that only turn are SVG files in `assets/`. The few that change
//! shape as they move are generated, one SVG per step of the change.
//!
//! A turned sprite's edges would sample the atlas tiles beside it, so every
//! sprite is padded with enough transparent margin that, turned any amount
//! about its center, its edges stay outside the region it is clipped to.
//!
//! Every SVG is 1×1 in size, whatever its view, so GPUI rasterizes it at
//! exactly the pixel size asked for, with no rounding to throw off where it
//! is drawn.
//!
//! GPUI loads every SVG through the app's [`AssetSource`], so spinners serve
//! theirs from [`SpinnerAssets`], which the app installs with
//! `Application::with_assets`. A generated sprite's asset path names it in a
//! registry of the shapes drawn so far; its markup is generated only when the
//! atlas misses, i.e. once per shape and size.

use std::borrow::Cow;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::fmt::Write;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::Once;
use std::sync::PoisonError;

use gpui::App;
use gpui::AssetSource;
use gpui::SharedString;

use crate::cap::Cap;
use crate::geom::Pt;
use crate::geom::centered_rect;
use crate::geom::pt;

const PREFIX: &str = "gpui-loading/";

const GENERATED: &str = "gpui-loading/generated/";

/// The margin every sprite's tile has around its view, as a fraction of the
/// view: enough that the tile, turned any amount, still covers the view and
/// the [`CLIP`] margin around it.
const MARGIN: f32 = 0.25;

/// How far past its view a sprite is drawn before being clipped, as a
/// fraction of the view, so shapes reaching the view's edge keep their
/// antialiased rim.
const CLIP: f32 = 0.03;

/// The SVG files in `assets/`, each a shape that only ever turns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Asset {
    /// `ClassicV2`'s and Compass's tick at 12 o'clock.
    Tick,
    /// Classic's bar at 3 o'clock.
    ClassicBar,
    ClockHand,
    RadarBeam,
    /// Comet's ring, fading in toward its head at 12 o'clock.
    Comet,
    /// Orbit's half ring, fading in from 6 o'clock round to 12.
    Orbit,
}

impl Asset {
    const ALL: [Asset; 6] = [
        Asset::Tick,
        Asset::ClassicBar,
        Asset::ClockHand,
        Asset::RadarBeam,
        Asset::Comet,
        Asset::Orbit,
    ];

    fn path(self) -> &'static str {
        match self {
            Asset::Tick => "gpui-loading/tick.svg",
            Asset::ClassicBar => "gpui-loading/classic-bar.svg",
            Asset::ClockHand => "gpui-loading/clock-hand.svg",
            Asset::RadarBeam => "gpui-loading/radar-beam.svg",
            Asset::Comet => "gpui-loading/comet.svg",
            Asset::Orbit => "gpui-loading/orbit.svg",
        }
    }

    fn bytes(self) -> &'static [u8] {
        match self {
            Asset::Tick => include_bytes!("../assets/tick.svg"),
            Asset::ClassicBar => include_bytes!("../assets/classic-bar.svg"),
            Asset::ClockHand => include_bytes!("../assets/clock-hand.svg"),
            Asset::RadarBeam => include_bytes!("../assets/radar-beam.svg"),
            Asset::Comet => include_bytes!("../assets/comet.svg"),
            Asset::Orbit => include_bytes!("../assets/orbit.svg"),
        }
    }

    fn view(self) -> f32 {
        match self {
            Asset::Tick | Asset::ClockHand | Asset::RadarBeam => 16.0,
            Asset::ClassicBar | Asset::Comet | Asset::Orbit => 1.0,
        }
    }
}

const PROBE: &str = "gpui-loading/probe";

/// A length in a sprite's view, in thousandths, so sprites can be hashed.
pub(crate) type Milli = i32;

#[expect(
    clippy::cast_possible_truncation,
    reason = "sprite lengths are small, so thousandths of them fit an i32"
)]
pub(crate) fn milli(value: f32) -> Milli {
    (value * 1000.0).round() as Milli
}

#[expect(
    clippy::cast_precision_loss,
    reason = "sprite lengths are small, so their thousandths are exact in an f32"
)]
fn unmilli(value: Milli) -> f32 {
    value as f32 / 1000.0
}

/// One shape, drawn in a square `view` units across that is laid over the
/// spinner. Lengths are in [`Milli`] units of that view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Sprite {
    /// One of the SVG files.
    Asset(Asset),
    /// A dash of `length` along a circle about the view's center, from
    /// 3 o'clock clockwise, as SVG strokes a `<circle>`.
    Arc {
        view: Milli,
        radius: Milli,
        width: Milli,
        length: Milli,
        cap: Cap,
    },
    /// Morph's rounded square.
    RoundedRect {
        view: Milli,
        width: Milli,
        height: Milli,
        radius: Milli,
    },
    /// Gather's four blocks, `pull` of the way in toward the center.
    GatherBlocks { pull: Milli },
    /// One of Atom's orbits, squashed to `squash` of its width.
    AtomOrbit { squash: Milli },
    /// Trace's dash, `offset` along its rounded square.
    TraceDash { offset: Milli, cap: Cap },
    /// Flip's face turned `degrees` about its horizontal axis, in perspective.
    FlipFace { degrees: i32 },
}

#[derive(Default)]
struct Registry {
    paths: HashMap<Sprite, SharedString>,
    sprites: HashMap<SharedString, Sprite>,
}

static REGISTRY: LazyLock<Mutex<Registry>> = LazyLock::new(Default::default);

impl Sprite {
    /// The asset path GPUI knows this sprite by.
    pub fn path(self) -> SharedString {
        if let Sprite::Asset(asset) = self {
            return SharedString::new_static(asset.path());
        }
        let mut registry = REGISTRY.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(path) = registry.paths.get(&self) {
            return path.clone();
        }
        let path = SharedString::from(format!("{GENERATED}{}", registry.paths.len()));
        registry.paths.insert(self, path.clone());
        registry.sprites.insert(path.clone(), self);
        path
    }

    fn lookup(path: &str) -> Option<Sprite> {
        let registry = REGISTRY.lock().unwrap_or_else(PoisonError::into_inner);
        registry.sprites.get(path).copied()
    }

    /// The side of the sprite's view.
    pub fn view(self) -> f32 {
        match self {
            Sprite::Asset(asset) => asset.view(),
            Sprite::Arc { view, .. } | Sprite::RoundedRect { view, .. } => unmilli(view),
            Sprite::TraceDash { .. } => TRACE_VIEW,
            Sprite::GatherBlocks { .. } | Sprite::AtomOrbit { .. } | Sprite::FlipFace { .. } => 1.0,
        }
    }

    /// The region the sprite is clipped to, and the padded one its tile
    /// covers, as `(clip, tile)` margins around the view, in view units.
    pub fn margins(self) -> (f32, f32) {
        let view = self.view();
        match self {
            // Turned 45° with its blocks drawn in, the group reaches 0.594
            // of the view from its center, so it is clipped 0.1 outside the
            // view, and its tile padded enough to cover that turned.
            Sprite::GatherBlocks { .. } => (0.1 * view, 0.35 * view),
            Sprite::Asset(_)
            | Sprite::Arc { .. }
            | Sprite::RoundedRect { .. }
            | Sprite::AtomOrbit { .. }
            | Sprite::TraceDash { .. }
            | Sprite::FlipFace { .. } => (CLIP * view, MARGIN * view),
        }
    }

    /// The sprite's SVG markup: an asset's file, or a generated sprite's
    /// shape.
    #[expect(
        clippy::let_underscore_must_use,
        reason = "writing to a String cannot fail"
    )]
    fn svg(self) -> String {
        let view = self.view();
        let (_, m) = self.margins();
        let side = view + 2.0 * m;
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="{} {} {side} {side}">"#,
            -m, -m,
        );
        let c = view / 2.0;
        match self {
            Sprite::Asset(asset) => return String::from_utf8_lossy(asset.bytes()).into_owned(),
            Sprite::Arc {
                radius,
                width,
                length,
                cap,
                ..
            } => {
                let (r, length) = (unmilli(radius), unmilli(length));
                let circumference = TAU * r;
                let dashes = if length < circumference {
                    format!(r#" stroke-dasharray="{length} {circumference}""#)
                } else {
                    String::new()
                };
                let _ = write!(
                    svg,
                    r#"<circle cx="{c}" cy="{c}" r="{r}" fill="none" stroke="black" stroke-width="{}" stroke-linecap="{}"{dashes}/>"#,
                    unmilli(width),
                    cap.svg(),
                );
            }
            Sprite::RoundedRect {
                width,
                height,
                radius,
                ..
            } => {
                let (w, h) = (unmilli(width), unmilli(height));
                let _ = write!(
                    svg,
                    r#"<rect x="{}" y="{}" width="{w}" height="{h}" rx="{}"/>"#,
                    c - w / 2.0,
                    c - h / 2.0,
                    unmilli(radius),
                );
            }
            Sprite::GatherBlocks { pull } => {
                for (corner, direction) in GATHER_BLOCKS {
                    let shift = GATHER_PULL * unmilli(pull);
                    let _ = write!(
                        svg,
                        r#"<rect x="{}" y="{}" width="{GATHER_BLOCK}" height="{GATHER_BLOCK}" rx="0.14"/>"#,
                        corner.x + direction.x * shift,
                        corner.y + direction.y * shift,
                    );
                }
            }
            Sprite::AtomOrbit { squash } => {
                // The orbit's border is squashed along with it, as a 3D
                // turn would.
                const STROKE: f32 = 0.045;
                let _ = write!(
                    svg,
                    r#"<circle transform="translate(0.5 0.5) scale({} 1)" r="{}" fill="none" stroke="black" stroke-width="{STROKE}"/>"#,
                    unmilli(squash),
                    0.5 - STROKE / 2.0,
                );
            }
            Sprite::TraceDash { offset, cap } => {
                let inset = (TRACE_VIEW - TRACE_SIDE) / 2.0;
                let _ = write!(
                    svg,
                    r#"<rect x="{inset}" y="{inset}" width="{TRACE_SIDE}" height="{TRACE_SIDE}" rx="{TRACE_RADIUS}" fill="none" stroke="black" stroke-width="{TRACE_STROKE}" stroke-linecap="{}" stroke-dasharray="{TRACE_DASH} {}" stroke-dashoffset="{}"/>"#,
                    cap.svg(),
                    TRACE_PERIMETER - TRACE_DASH,
                    -unmilli(offset),
                );
            }
            Sprite::FlipFace { degrees } => {
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "whole degrees within a turn are exact in an f32"
                )]
                let degrees = degrees as f32;
                let face = centered_rect(Pt::default(), FLIP_FACE, FLIP_FACE, 0.06);
                polygon(
                    &mut svg,
                    face.into_iter().map(|p| flip_project(p, degrees)),
                    1.0,
                );
            }
        }
        svg.push_str("</svg>");
        svg
    }
}

/// Appends a filled polygon.
#[expect(
    clippy::let_underscore_must_use,
    reason = "writing to a String cannot fail"
)]
fn polygon(svg: &mut String, points: impl IntoIterator<Item = Pt>, alpha: f32) {
    let _ = write!(svg, r#"<path fill-opacity="{alpha:.4}" d=""#);
    for (i, p) in points.into_iter().enumerate() {
        let _ = write!(
            svg,
            "{}{:.4} {:.4}",
            if i == 0 { "M" } else { "L" },
            p.x,
            p.y
        );
    }
    svg.push_str(r#"Z"/>"#);
}

impl Cap {
    fn svg(self) -> &'static str {
        match self {
            Cap::Flat => "butt",
            Cap::Round => "round",
        }
    }
}

pub(crate) const TRACE_VIEW: f32 = 20.0;
pub(crate) const TRACE_SIDE: f32 = 17.5;
pub(crate) const TRACE_RADIUS: f32 = 4.0;
pub(crate) const TRACE_STROKE: f32 = 2.5;
const TRACE_DASH: f32 = 16.0;
pub(crate) const TRACE_PERIMETER: f32 =
    4.0 * (TRACE_SIDE - 2.0 * TRACE_RADIUS) + TAU * TRACE_RADIUS;

const GATHER_BLOCK: f32 = (1.0 - GATHER_GAP) / 2.0;

const GATHER_GAP: f32 = 0.24;

/// How far each block travels toward the center: enough to close the gap
/// to 8%, rounded as the web version rounds it, to 21.1% of a block.
const GATHER_PULL: f32 = 0.211 * GATHER_BLOCK;

/// Top-left corner of each of Gather's blocks, and the direction it is
/// pulled in.
const GATHER_BLOCKS: [(Pt, Pt); 4] = [
    (pt(0.0, 0.0), pt(1.0, 1.0)),
    (pt(GATHER_BLOCK + GATHER_GAP, 0.0), pt(-1.0, 1.0)),
    (pt(0.0, GATHER_BLOCK + GATHER_GAP), pt(1.0, -1.0)),
    (
        pt(GATHER_BLOCK + GATHER_GAP, GATHER_BLOCK + GATHER_GAP),
        pt(-1.0, -1.0),
    ),
];

const FLIP_FACE: f32 = 0.64;

/// Distance to the viewer, as `perspective: calc(size * 3)`.
const FLIP_PERSPECTIVE: f32 = 3.0;

/// `rotateX(degrees)` under perspective, for a point on Flip's face
/// relative to its center, placed in the unit view.
fn flip_project(p: Pt, degrees: f32) -> Pt {
    let (sin, cos) = degrees.to_radians().sin_cos();
    let (y, z) = (p.y * cos, p.y * sin);
    let scale = FLIP_PERSPECTIVE / (FLIP_PERSPECTIVE - z);
    Pt {
        x: 0.5 + p.x * scale,
        y: 0.5 + y * scale,
    }
}

/// Serves the spinners' sprites, and everything else from the app's own
/// source. Install it when creating the application:
///
/// ```ignore
/// Application::new().with_assets(SpinnerAssets::new())
/// // or, with assets of your own:
/// Application::new().with_assets(SpinnerAssets::wrap(MyAssets))
/// ```
pub struct SpinnerAssets<A: AssetSource = ()>(A);

impl SpinnerAssets {
    /// The spinners' assets alone, for an app with none of its own.
    #[must_use]
    pub fn new() -> Self {
        SpinnerAssets(())
    }
}

impl Default for SpinnerAssets {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: AssetSource> SpinnerAssets<A> {
    /// The spinners' assets, falling back to `inner` for everything else.
    pub fn wrap(inner: A) -> Self {
        SpinnerAssets(inner)
    }
}

impl<A: AssetSource> AssetSource for SpinnerAssets<A> {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path == PROBE {
            return Ok(Some(Cow::Borrowed(&[])));
        }
        if path.starts_with(GENERATED) {
            return Ok(Sprite::lookup(path).map(|sprite| Cow::Owned(sprite.svg().into_bytes())));
        }
        if path.starts_with(PREFIX) {
            let asset = Asset::ALL.into_iter().find(|asset| asset.path() == path);
            return Ok(asset.map(|asset| Cow::Borrowed(asset.bytes())));
        }
        self.0.load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        self.0.list(path)
    }
}

/// Complains, once, if the app has not installed [`SpinnerAssets`] — without
/// them every sprite silently fails to load.
pub(crate) fn check_assets(cx: &App) {
    static CHECK: Once = Once::new();
    CHECK.call_once(|| {
        if !matches!(cx.asset_source().load(PROBE), Ok(Some(_))) {
            log::error!(
                "gpui-loading: spinners need their assets. Create the app with \
                 `Application::new().with_assets(gpui_loading::SpinnerAssets::new())`, \
                 or `SpinnerAssets::wrap(your_assets)`."
            );
            debug_assert!(false, "gpui-loading: SpinnerAssets are not installed");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn renders(sprite: Sprite) {
        let svg = sprite.svg();
        assert!(svg.starts_with("<svg") && svg.ends_with("</svg>"), "{svg}");
        assert!(!svg.contains("NaN") && !svg.contains("inf"), "{svg}");
    }

    #[test]
    fn every_generated_sprite_is_well_formed() {
        renders(Sprite::Arc {
            view: milli(24.0),
            radius: milli(10.0),
            width: milli(2.5),
            length: milli(18.0),
            cap: Cap::Round,
        });
        renders(Sprite::RoundedRect {
            view: milli(1.0),
            width: milli(0.64),
            height: milli(0.64),
            radius: milli(0.08),
        });
        renders(Sprite::TraceDash {
            offset: milli(3.0),
            cap: Cap::Flat,
        });
        renders(Sprite::GatherBlocks { pull: milli(0.5) });
        renders(Sprite::AtomOrbit { squash: milli(0.3) });
        renders(Sprite::FlipFace { degrees: -45 });
        renders(Sprite::FlipFace { degrees: -90 });
    }

    #[test]
    fn asset_files_are_padded_like_generated_sprites() {
        for asset in Asset::ALL {
            let svg = std::str::from_utf8(asset.bytes()).expect("asset files are UTF-8");
            let (_, m) = Sprite::Asset(asset).margins();
            let side = asset.view() + 2.0 * m;
            let header = format!(
                r#"width="1" height="1" viewBox="{} {} {} {}""#,
                -m, -m, side, side
            );
            assert!(svg.contains(&header), "{asset:?} lacks {header}");
        }
    }

    #[test]
    fn a_turned_tile_covers_its_clip() {
        for sprite in [
            Sprite::Asset(Asset::Comet),
            Sprite::Asset(Asset::Tick),
            Sprite::GatherBlocks { pull: 0 },
            Sprite::AtomOrbit { squash: 1000 },
        ] {
            let (clip, tile) = sprite.margins();
            let half = sprite.view() / 2.0;
            // The tile's inscribed circle contains the clip's corners.
            assert!(half + tile >= std::f32::consts::SQRT_2 * (half + clip));
        }
    }

    #[test]
    fn paths_are_stable_and_round_trip() {
        let sprite = Sprite::FlipFace { degrees: -12 };
        let path = sprite.path();
        assert_eq!(sprite.path(), path);
        assert_eq!(Sprite::lookup(&path), Some(sprite));
    }

    #[test]
    fn assets_serve_sprites_and_defer_the_rest() {
        let assets = SpinnerAssets::new();
        for sprite in [
            Sprite::Asset(Asset::Orbit),
            Sprite::AtomOrbit { squash: 500 },
        ] {
            assert!(matches!(assets.load(&sprite.path()), Ok(Some(_))));
        }
        assert!(matches!(assets.load("icons/other.svg"), Ok(None)));
        assert!(matches!(assets.load(PROBE), Ok(Some(_))));
    }
}
