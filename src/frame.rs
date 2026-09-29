//! What every spinner shares: its props, its root element, and the painter
//! it draws with.

use std::time::Duration;

use gpui::App;
use gpui::BorderStyle;
use gpui::Bounds;
use gpui::ContentMask;
use gpui::Div;
use gpui::ElementId;
use gpui::Hsla;
use gpui::ParentElement;
use gpui::PathBuilder;
use gpui::Pixels;
use gpui::Point;
use gpui::Refineable;
use gpui::StyleRefinement;
use gpui::Styled;
use gpui::TransformationMatrix;
use gpui::Window;
use gpui::canvas;
use gpui::div;
use gpui::point;
use gpui::px;
use gpui::quad;
use gpui::size;
use gpui::transparent_black;

use crate::geom::Affine;
use crate::geom::Pt;
use crate::motion::Clock;
use crate::motion::DEFAULT_SIZE;
use crate::motion::Motion;
use crate::motion::PlayState;
use crate::motion::ReducedMotion;
use crate::motion::SpinnerName;
use crate::motion::reduced_motion;
use crate::sprite::Asset;
use crate::sprite::Sprite;
use crate::sprite::check_assets;

/// The props every spinner accepts.
pub struct SpinnerProps {
    id: ElementId,
    size: Option<Pixels>,
    color: Option<Hsla>,
    duration: Option<Duration>,
    play_state: Option<PlayState>,
    style: StyleRefinement,
}

impl SpinnerProps {
    pub(crate) fn new(id: impl Into<ElementId>) -> SpinnerProps {
        SpinnerProps {
            id: id.into(),
            size: None,
            color: None,
            duration: None,
            play_state: None,
            style: StyleRefinement::default(),
        }
    }

    pub(crate) fn set_size(&mut self, size: Pixels) {
        self.size = Some(size);
    }

    pub(crate) fn set_color(&mut self, color: Hsla) {
        self.color = Some(color);
    }

    pub(crate) fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    pub(crate) fn set_play_state(&mut self, play_state: PlayState) {
        self.play_state = Some(play_state);
    }

    pub(crate) fn style_mut(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// A spinner's constructors, and the builder methods every spinner shares.
/// A spinner with no options of its own is a tuple struct around its props;
/// one with options names them after the type.
macro_rules! spinner_props {
    ($ty:ident) => {
        impl $ty {
            /// A spinner with default props. `id` keys its animation state, so
            /// it must be unique among its siblings, as for any stateful GPUI
            /// element.
            pub fn new(id: impl Into<gpui::ElementId>) -> Self {
                Self::from_props($crate::frame::SpinnerProps::new(id))
            }

            pub(crate) fn from_props(props: $crate::frame::SpinnerProps) -> Self {
                Self(props)
            }
        }

        $crate::frame::shared_props!($ty, 0);
    };
    ($ty:ident, $($option:ident),+) => {
        impl $ty {
            /// A spinner with default props. `id` keys its animation state, so
            /// it must be unique among its siblings, as for any stateful GPUI
            /// element.
            pub fn new(id: impl Into<gpui::ElementId>) -> Self {
                Self::from_props($crate::frame::SpinnerProps::new(id))
            }

            pub(crate) fn from_props(props: $crate::frame::SpinnerProps) -> Self {
                Self {
                    props,
                    $($option: Default::default(),)+
                }
            }
        }

        $crate::frame::shared_props!($ty, props);
    };
}

/// Builder methods for the props every spinner shares, plus [`Styled`] so a
/// spinner can take margins, opacity and the like. `$props` names the field
/// holding the spinner's [`SpinnerProps`].
macro_rules! shared_props {
    ($ty:ident, $props:tt) => {
        impl $ty {
            /// The spinner's size in pixels. Defaults to 20.
            #[must_use]
            pub fn size(mut self, size: impl Into<gpui::Pixels>) -> Self {
                self.$props.set_size(size.into());
                self
            }

            /// The colour to paint with. Defaults to the inherited text colour.
            #[must_use]
            pub fn color(mut self, color: impl Into<gpui::Hsla>) -> Self {
                self.$props.set_color(color.into());
                self
            }

            /// The length of one cycle. Defaults to the spinner's own duration.
            #[must_use]
            pub fn duration(mut self, duration: std::time::Duration) -> Self {
                self.$props.set_duration(duration);
                self
            }

            #[must_use]
            pub fn play_state(mut self, play_state: $crate::PlayState) -> Self {
                self.$props.set_play_state(play_state);
                self
            }
        }

        impl gpui::Styled for $ty {
            fn style(&mut self) -> &mut gpui::StyleRefinement {
                self.$props.style_mut()
            }
        }
    };
}

macro_rules! easing_prop {
    ($ty:ident) => {
        impl $ty {
            #[must_use]
            pub fn easing(mut self, easing: $crate::Easing) -> Self {
                self.easing = easing;
                self
            }
        }
    };
}

macro_rules! cap_prop {
    ($ty:ident) => {
        impl $ty {
            #[must_use]
            pub fn cap(mut self, cap: $crate::Cap) -> Self {
                self.cap = cap;
                self
            }
        }
    };
}

pub(crate) use cap_prop;
pub(crate) use easing_prop;
pub(crate) use shared_props;
pub(crate) use spinner_props;

/// A spinner's root element: sized from `size` and `aspect` (width and
/// height as multiples of the size), styled by the caller, and painted by
/// `paint` from the spinner's current [`Motion`].
pub(crate) fn frame(
    props: SpinnerProps,
    name: SpinnerName,
    aspect: (f32, f32),
    window: &mut Window,
    cx: &mut App,
    paint: impl Fn(&mut Painter, Motion) + 'static,
) -> Div {
    let size = props.size.unwrap_or(px(DEFAULT_SIZE));
    let duration = props.duration.unwrap_or(name.default_duration());
    let preference = reduced_motion(cx);
    check_assets(cx);
    // Reduced motion holds every clock still, as a pause would.
    let play_state = match preference {
        ReducedMotion::Reduce => PlayState::Paused,
        ReducedMotion::NoPreference => props.play_state.unwrap_or_default(),
    };

    let clock = window.use_keyed_state(props.id, cx, |_, _| Clock::default());
    let elapsed = clock.update(cx, |clock, _| clock.tick(play_state));
    if play_state == PlayState::Running {
        window.request_animation_frame();
    }
    let motion = Motion::new(elapsed, duration, preference);
    let color = props.color;

    let mut root = div().flex_none().w(size * aspect.0).h(size * aspect.1);
    root.style().refine(&props.style);
    root.child(
        canvas(
            |_, _, _| {},
            move |bounds, (), window, cx| {
                let color = color.unwrap_or_else(|| window.text_style().color);
                let size = f32::from(size);
                let mut painter = Painter {
                    window,
                    cx,
                    origin: bounds.origin,
                    box_size: bounds.size,
                    size,
                    unit: size,
                    color,
                };
                paint(&mut painter, motion);
            },
        )
        .size_full(),
    )
}

/// Paints a spinner in its own coordinate space, with the primitives GPUI
/// draws cheaply: quads for anything round or rectangular, and cached
/// sprites for continuous transforms. Wave's changing capsules use one path.
pub(crate) struct Painter<'a> {
    window: &'a mut Window,
    cx: &'a App,
    origin: Point<Pixels>,
    box_size: gpui::Size<Pixels>,
    size: f32,
    unit: f32,
    color: Hsla,
}

impl Painter<'_> {
    /// Sets the coordinate space so that `extent` units span the spinner's
    /// size, like an SVG `viewBox` of `0 0 extent extent`. Until called,
    /// one unit is the whole size.
    pub fn view(&mut self, extent: f32) {
        self.unit = self.size / extent;
    }

    /// The spinner's size in pixels.
    pub fn size(&self) -> f32 {
        self.size
    }

    /// The spinner's size in device pixels. `view` this many units to lay
    /// shapes out in whole device pixels.
    pub fn device_size(&self) -> f32 {
        self.size * self.window.scale_factor()
    }

    fn bounds(&self, x: f32, y: f32, width: f32, height: f32) -> Bounds<Pixels> {
        Bounds::new(
            point(
                self.origin.x + px(x * self.unit),
                self.origin.y + px(y * self.unit),
            ),
            size(px(width * self.unit), px(height * self.unit)),
        )
    }

    fn quad(&mut self, bounds: Bounds<Pixels>, radius: f32, border: f32, alpha: f32) {
        if alpha <= 0.0 || bounds.size.width <= px(0.0) || bounds.size.height <= px(0.0) {
            return;
        }
        let color = self.color.opacity(alpha);
        let (background, border_color) = if border > 0.0 {
            (transparent_black(), color)
        } else {
            (color, transparent_black())
        };
        self.window.paint_quad(quad(
            bounds,
            px(radius * self.unit),
            background,
            px(border * self.unit),
            border_color,
            BorderStyle::Solid,
        ));
    }

    /// A stationary rectangle. Moving or resizing shapes must use sprites
    /// or paths, because GPUI snaps quad bounds to device pixels.
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32, alpha: f32) {
        let bounds = self.bounds(x, y, width, height);
        self.quad(bounds, radius, 0.0, alpha);
    }

    /// A dot that doesn't move, of about the given radius, sized and
    /// placed in whole device pixels so its edges land on them: a small dot
    /// between pixels blurs out of round. Dots placed symmetrically about
    /// the spinner's center stay so, and a dot on the center stays on it,
    /// concentric with anything turning or growing about it.
    pub fn dot(&mut self, center: Pt, radius: f32, alpha: f32) {
        let scale = self.window.scale_factor();
        let diameter = 2.0 * radius * self.unit * scale;
        let middle = self.size / self.unit / 2.0;
        let side = if (center.x - middle).abs() < 1e-3 && (center.y - middle).abs() < 1e-3 {
            pixel_diameter(
                diameter,
                (f32::from(self.origin.y) + self.size / 2.0) * scale,
            )
        } else {
            diameter.round().max(1.0)
        };
        let (x, y) = (
            (f32::from(self.origin.x) + center.x * self.unit) * scale,
            (f32::from(self.origin.y) + center.y * self.unit) * scale,
        );
        let (left, top) = ((x - side / 2.0).round(), (y - side / 2.0).round());
        let bounds = Bounds::new(
            point(px(left / scale), px(top / scale)),
            size(px(side / scale), px(side / scale)),
        );
        self.quad(bounds, side / 2.0 / scale / self.unit, 0.0, alpha);
    }

    /// A stationary circle; use [`Self::animated_circle`] for motion.
    pub fn circle(&mut self, center: Pt, radius: f32, alpha: f32) {
        let d = radius * 2.0;
        self.rect(center.x - radius, center.y - radius, d, d, radius, alpha);
    }

    /// A moving or growing circle, transformed continuously on the GPU.
    /// Its sprite identity and raster size do not depend on animation time.
    pub fn animated_circle(&mut self, center: Pt, radius: f32, alpha: f32) {
        if radius <= 0.0 || alpha <= 0.0 {
            return;
        }
        let unit = self.unit;
        let k = unit / self.size;
        let scale = 2.0 * radius * k;
        let transform = Affine::linear(scale, 0.0, 0.0, scale)
            .about(crate::geom::pt(0.5, 0.5))
            .then(Affine::translate(center.x * k - 0.5, center.y * k - 0.5));
        self.view(1.0);
        self.sprite(Sprite::Asset(Asset::Circle), transform, alpha);
        self.unit = unit;
    }

    /// Changing-height pills with circular ends, batched into one path.
    /// Scaling a fixed pill would squash its ends; changing quad bounds snaps.
    pub fn capsules(&mut self, bars: impl IntoIterator<Item = (f32, f32, f32, f32)>) {
        let mut path = PathBuilder::fill();
        for (x, y, width, height) in bars {
            append_capsule(&mut path, self.bounds(x, y, width, height));
        }
        match path.build() {
            Ok(path) => self.window.paint_path(path, self.color),
            Err(error) => log::warn!("gpui-loading: failed to build capsules: {error}"),
        }
    }

    /// The outline of a square `side` across centered on `center`, drawn
    /// inside its bounds like a CSS border.
    pub fn outline(&mut self, center: Pt, side: f32, radius: f32, stroke: f32, alpha: f32) {
        let half = side / 2.0;
        let bounds = self.bounds(center.x - half, center.y - half, side, side);
        self.quad(bounds, radius, stroke, alpha);
    }

    /// A ring of the given outer radius, `stroke` thick.
    pub fn ring(&mut self, center: Pt, radius: f32, stroke: f32, alpha: f32) {
        self.outline(center, radius * 2.0, radius, stroke, alpha);
    }

    /// Draws `sprite` over the spinner's square box, transformed by `turn`.
    /// The transformed shape must fit within its view and clip margin.
    pub fn sprite(&mut self, sprite: Sprite, turn: Affine, alpha: f32) {
        if alpha <= 0.0 {
            return;
        }
        let view = sprite.view();
        debug_assert!((self.size / self.unit - view).abs() < 1e-3);
        let (clip, tile) = sprite.margins();
        // The canvas can be wider than a square (e.g. BouncingDots).
        // Clip to that entire box, not to the sprite's untransformed view.
        let margin = px(clip * self.unit);
        let clip = Bounds::new(
            self.origin - point(margin, margin),
            self.box_size + size(margin * 2.0, margin * 2.0),
        );
        let scale = self.window.scale_factor();
        let origin = (
            f32::from(self.origin.x) * scale,
            f32::from(self.origin.y) * scale,
        );
        let (bounds, transformation) =
            sprite_tile(origin, self.unit * scale, scale, view, tile, turn);
        let color = self.color.opacity(alpha);
        let cx = self.cx;
        self.window
            .with_content_mask(Some(ContentMask { bounds: clip }), |window| {
                if let Err(error) =
                    window.paint_svg(bounds, sprite.path(), None, transformation, color, cx)
                {
                    log::warn!("gpui-loading: failed to paint sprite {sprite:?}: {error}");
                }
            });
    }
}

/// A vertical pill with semicircular ends, in logical pixels. No rounding:
/// its top and bottom must follow Wave's changing height between pixels.
fn append_capsule(path: &mut PathBuilder, bounds: Bounds<Pixels>) {
    let radius = bounds.size.width / 2.0;
    let left = bounds.left();
    let right = bounds.right();
    let top = bounds.top();
    let bottom = bounds.bottom();
    let radii = point(radius, radius);
    path.move_to(point(left, top + radius));
    path.arc_to(radii, px(0.0), false, true, point(right, top + radius));
    path.line_to(point(right, bottom - radius));
    path.arc_to(radii, px(0.0), false, true, point(left, bottom - radius));
    path.close();
}

/// Lays out the tile of a sprite `view` units across, padded by `tile`, over
/// a spinner at device `origin` with `k` device pixels per view unit, and
/// the transformation turning it by `turn` there.
///
/// GPUI rasterizes a sprite at a whole number of pixels and rounds where it
/// lands, which would leave it up to a pixel off the center it turns about —
/// wobbling as it spins, and popping when a turn wraps. So the tile is laid
/// out in whole device pixels, just covering the padded view, leaving GPUI
/// nothing to round, and the view is mapped onto it exactly.
#[expect(
    clippy::many_single_char_names,
    reason = "the names follow the affine matrix entries they hold"
)]
fn sprite_tile(
    origin: (f32, f32),
    k: f32,
    scale: f32,
    view: f32,
    tile: f32,
    turn: Affine,
) -> (Bounds<Pixels>, TransformationMatrix) {
    let (ox, oy) = origin;
    let padded = view + 2.0 * tile;
    let side = (k * padded).ceil() + 1.0;
    let inset = k * (view - padded) / 2.0;
    let (left, top) = ((ox + inset).floor(), (oy + inset).floor());
    // A hair under `side`, so GPUI's ceil lands on `side` exactly.
    let bounds = Bounds::new(
        point(px(left / scale), px(top / scale)),
        size(px((side - 0.01) / scale), px((side - 0.01) / scale)),
    );
    // The tile puts view point u at q = (left, top) + s·(u + tile). The
    // sprite shader transforms device positions, so map each q back to its
    // u and turn it into device space: q ↦ o + k·(L·u + t).
    let s = side / padded;
    let (ux, uy) = (-left / s - tile, -top / s - tile);
    let [[a, c], [b, d]] = turn.linear_part();
    let (tx, ty) = turn.translation();
    let r = k / s;
    let transformation = TransformationMatrix {
        rotation_scale: [[r * a, r * c], [r * b, r * d]],
        translation: [
            ox + k * (a * ux + c * uy + tx),
            oy + k * (b * ux + d * uy + ty),
        ],
    };
    (bounds, transformation)
}

/// The whole number of device pixels nearest `diameter` that a dot centered
/// at device `center` can span with its edges on pixels: even about a pixel
/// edge, odd about a pixel's middle, the larger when two are as near.
fn pixel_diameter(diameter: f32, center: f32) -> f32 {
    let nearest = diameter.round().max(1.0);
    let twice = 2.0 * center;
    if (twice - twice.round()).abs() > 1e-3 {
        return nearest;
    }
    if (nearest - twice.round()).rem_euclid(2.0) == 0.0 {
        return nearest;
    }
    if nearest - diameter > 0.0 && nearest > 1.0 {
        nearest - 1.0
    } else {
        nearest + 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::pt;

    /// Where GPUI's `paint_svg` draws a tile laid out over `bounds`, as
    /// its device origin and side, for an SVG 1×1 in size.
    fn drawn(bounds: Bounds<Pixels>, scale: f32) -> (f32, f32, f32) {
        let x = f32::from(bounds.origin.x) * scale;
        let y = f32::from(bounds.origin.y) * scale;
        let side = f32::from(bounds.size.width) * scale;
        // Rasterized at twice the size, then drawn at half that.
        let drawn = (side * 2.0).ceil() / 2.0;
        let place = |start: f32| (start + side / 2.0 - drawn / 2.0).round();
        (place(x), place(y), drawn)
    }

    #[test]
    #[expect(clippy::float_cmp, reason = "whole pixel counts are exact")]
    fn dots_span_whole_pixels_about_the_center() {
        // About a pixel edge, then about a pixel's middle.
        assert_eq!(pixel_diameter(5.0, 10.0), 6.0);
        assert_eq!(pixel_diameter(5.0, 10.5), 5.0);
        assert_eq!(pixel_diameter(10.0, 20.0), 10.0);
        assert_eq!(pixel_diameter(4.6, 10.0), 4.0);
        assert_eq!(pixel_diameter(5.4, 10.0), 6.0);
        assert_eq!(pixel_diameter(0.4, 10.5), 1.0);
        assert_eq!(pixel_diameter(0.4, 10.0), 2.0);
        // Off the half-pixel grid, the center can't be kept anyway.
        assert_eq!(pixel_diameter(5.0, 10.3), 5.0);
    }

    #[test]
    fn sprites_land_where_they_are_transformed() {
        let (view, tile) = (1.0, 0.25);
        for scale in [1.0, 1.25, 1.5, 2.0] {
            for size in [20.0, 32.0, 48.0, 64.0, 37.3] {
                for origin in [(0.0, 0.0), (101.0, 57.0), (101.3, 57.7), (640.5, 12.25)] {
                    for turn in [
                        Affine::IDENTITY,
                        Affine::rotate(37.0).about(pt(0.5, 0.5)),
                        Affine::rotate(90.0).about(pt(0.5, 0.5)),
                        Affine::linear(1.0, 0.0, 0.0, -1.0).about(pt(0.5, 0.5)),
                        Affine::linear(0.001, 0.0, 0.0, 0.001).about(pt(0.5, 0.5)),
                        Affine::linear(0.333, 0.0, 0.0, 0.333).about(pt(0.5, 0.5)),
                        Affine::linear(0.667, 0.0, 0.0, 0.667).about(pt(0.5, 0.5)),
                        Affine::translate(0.3, 0.0),
                        Affine::translate(-0.3, 0.0),
                        Affine::linear(1.3, 0.0, 0.0, 1.3)
                            .about(pt(0.5, 0.5))
                            .then(Affine::translate(0.123, 0.0)),
                        Affine::linear(0.7, 0.0, 0.0, 0.7)
                            .about(pt(0.5, 0.5))
                            .then(Affine::translate(-0.123, 0.0)),
                    ] {
                        let o = (origin.0 * scale, origin.1 * scale);
                        let k = size * scale;
                        let (bounds, transformation) = sprite_tile(o, k, scale, view, tile, turn);
                        let (left, top, side) = drawn(bounds, scale);
                        assert!(side >= k * (view + 2.0 * tile), "the tile covers the view");
                        // Each view point, as the tile holds it, lands where
                        // the turn puts it.
                        for u in [pt(0.0, 0.0), pt(0.5, 0.5), pt(1.0, 0.25), pt(0.8, 1.0)] {
                            let s = side / (view + 2.0 * tile);
                            let q = point(px(left + s * (u.x + tile)), px(top + s * (u.y + tile)));
                            let landed = transformation.apply(q);
                            let want = turn.apply(u);
                            let want = (o.0 + k * want.x, o.1 + k * want.y);
                            let error = (f32::from(landed.x) - want.0)
                                .abs()
                                .max((f32::from(landed.y) - want.1).abs());
                            assert!(
                                error < 1e-2,
                                "scale {scale}, size {size}, origin {origin:?}: {u:?} is {error}px off"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn animated_frames_reuse_the_tile_and_move_between_device_pixels() {
        let (origin, k, scale) = ((101.3, 57.7), 40.0, 2.0);
        let mut previous: Option<f32> = None;
        let mut tile_bounds = None;
        for frame in 0_u16..100 {
            let shift = f32::from(frame) * 0.001;
            let transform = Affine::linear(0.22 + shift, 0.0, 0.0, 0.22 + shift)
                .about(pt(0.5, 0.5))
                .then(Affine::translate(shift, -shift));
            let (bounds, matrix) = sprite_tile(origin, k, scale, 1.0, 0.25, transform);
            if let Some(tile_bounds) = tile_bounds {
                assert_eq!(bounds, tile_bounds, "animation cannot change the atlas key");
            }
            tile_bounds = Some(bounds);
            let (left, top, side) = drawn(bounds, scale);
            let center = point(px(left + side / 2.0), px(top + side / 2.0));
            let x = f32::from(matrix.apply(center).x);
            if let Some(previous) = previous {
                assert!((x - previous - 0.04_f32).abs() < 1e-3);
            }
            previous = Some(x);
        }
    }

    #[test]
    fn wave_capsules_keep_fractional_edges_and_round_ends() {
        for height in [6.13, 6.27, 12.37, 20.0] {
            for top in [0.37, (20.0 - height) / 2.0, 20.0 - height] {
                let bounds = Bounds::new(point(px(0.23), px(top)), size(px(2.4), px(height)));
                let mut builder = PathBuilder::fill();
                append_capsule(&mut builder, bounds);
                let path = builder.build().expect("a capsule tessellates");
                // Tessellation approximates the curved rim within 0.1 px.
                assert!((path.bounds.left() - bounds.left()).abs() < px(0.1));
                assert!((path.bounds.right() - bounds.right()).abs() < px(0.1));
                assert!((path.bounds.top() - bounds.top()).abs() < px(0.1));
                assert!((path.bounds.bottom() - bounds.bottom()).abs() < px(0.1));
                assert!(path.vertices.iter().all(|vertex| {
                    let x = f32::from(vertex.xy_position.x - bounds.center().x);
                    let y = f32::from(vertex.xy_position.y);
                    let cap_center = if y < top + 1.2 {
                        top + 1.2
                    } else if y > top + height - 1.2 {
                        top + height - 1.2
                    } else {
                        y
                    };
                    x * x + (y - cap_center).powi(2) <= 1.2_f32.powi(2) + 1e-3
                }));
            }
        }
    }
}
