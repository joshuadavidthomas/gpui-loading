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
/// sprites for everything else. Never paths — each batch of those costs a
/// full-window multisampled pass.
pub(crate) struct Painter<'a> {
    window: &'a mut Window,
    cx: &'a App,
    origin: Point<Pixels>,
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

    pub fn circle(&mut self, center: Pt, radius: f32, alpha: f32) {
        let d = radius * 2.0;
        self.rect(center.x - radius, center.y - radius, d, d, radius, alpha);
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

    /// Draws `sprite` over the spinner's square box, turned by `turn` —
    /// which may only rotate or mirror it about the box's center, so its
    /// padded tile always covers the region it is clipped to.
    pub fn sprite(&mut self, sprite: Sprite, turn: Affine, alpha: f32) {
        if alpha <= 0.0 {
            return;
        }
        let view = sprite.view();
        debug_assert!((self.size / self.unit - view).abs() < 1e-3);
        debug_assert!({
            let c = Pt {
                x: view / 2.0,
                y: view / 2.0,
            };
            let moved = turn.apply(c);
            (moved.x - c.x).abs() < 1e-3 && (moved.y - c.y).abs() < 1e-3
        });
        let (clip, tile) = sprite.margins();
        let clip = self.bounds(-clip, -clip, view + 2.0 * clip, view + 2.0 * clip);
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
                    window.paint_svg(bounds, sprite.path(), transformation, color, cx)
                {
                    log::warn!("gpui-loading: failed to paint sprite {sprite:?}: {error}");
                }
            });
    }
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
    fn sprites_land_where_they_are_turned() {
        let (view, tile) = (1.0, 0.25);
        for scale in [1.0, 1.25, 1.5, 2.0] {
            for size in [20.0, 32.0, 48.0, 64.0, 37.3] {
                for origin in [(0.0, 0.0), (101.0, 57.0), (101.3, 57.7), (640.5, 12.25)] {
                    for turn in [
                        Affine::IDENTITY,
                        Affine::rotate(37.0).about(pt(0.5, 0.5)),
                        Affine::rotate(90.0).about(pt(0.5, 0.5)),
                        Affine::linear(1.0, 0.0, 0.0, -1.0).about(pt(0.5, 0.5)),
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
}
