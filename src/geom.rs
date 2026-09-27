//! Points and affine transforms in a spinner's own coordinate space, and the
//! little polygon flattening the generated sprites need.

use std::f32::consts::TAU;

/// Segments used for a full turn of any curve.
const TURN_SEGMENTS: f32 = 128.0;

/// A point in a spinner's coordinate space.
pub type Pt = gpui::Point<f32>;

pub use gpui::point as pt;

/// A 2D affine transform: `x' = a·x + c·y + e`, `y' = b·x + d·y + f`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

impl Affine {
    pub const IDENTITY: Affine = Affine {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 0.0,
        f: 0.0,
    };

    pub fn linear(a: f32, b: f32, c: f32, d: f32) -> Affine {
        Affine {
            a,
            b,
            c,
            d,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn translate(x: f32, y: f32) -> Affine {
        Affine {
            e: x,
            f: y,
            ..Self::IDENTITY
        }
    }

    /// Clockwise on screen, like CSS `rotate()`.
    pub fn rotate(degrees: f32) -> Affine {
        let (sin, cos) = degrees.to_radians().sin_cos();
        Affine::linear(cos, sin, -sin, cos)
    }

    /// Applies `self` first, then `next`.
    pub fn then(self, next: Affine) -> Affine {
        Affine {
            a: next.a * self.a + next.c * self.b,
            b: next.b * self.a + next.d * self.b,
            c: next.a * self.c + next.c * self.d,
            d: next.b * self.c + next.d * self.d,
            e: next.a * self.e + next.c * self.f + next.e,
            f: next.b * self.e + next.d * self.f + next.f,
        }
    }

    /// `self` with `origin` as its fixed point, like CSS `transform-origin`.
    pub fn about(self, origin: Pt) -> Affine {
        Affine::translate(-origin.x, -origin.y)
            .then(self)
            .then(Affine::translate(origin.x, origin.y))
    }

    /// The 2×2 linear part, row-major.
    pub fn linear_part(&self) -> [[f32; 2]; 2] {
        [[self.a, self.c], [self.b, self.d]]
    }

    pub fn translation(&self) -> (f32, f32) {
        (self.e, self.f)
    }

    pub fn apply(&self, p: Pt) -> Pt {
        pt(
            self.a * p.x + self.c * p.y + self.e,
            self.b * p.x + self.d * p.y + self.f,
        )
    }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the count is a whole number of at least 2, at most 128 a turn"
)]
fn segments(sweep_radians: f32) -> usize {
    ((sweep_radians.abs() / TAU) * TURN_SEGMENTS)
        .ceil()
        .max(2.0) as usize
}

/// Points along a circular arc. Angles are in degrees, measured clockwise
/// on screen from the positive x axis — the direction SVG strokes a circle.
#[expect(
    clippy::cast_precision_loss,
    reason = "an arc has at most 128 segments a turn, exact in an f32"
)]
pub fn arc(center: Pt, radius: f32, from: f32, to: f32) -> Vec<Pt> {
    let (from, to) = (from.to_radians(), to.to_radians());
    let n = segments(to - from);
    (0..=n)
        .map(|i| {
            let angle = from + (to - from) * i as f32 / n as f32;
            pt(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        })
        .collect()
}

/// A rounded rectangle traced the way SVG traces `<rect rx>`: from the end
/// of the top-left corner, clockwise. The radius is clamped like CSS clamps
/// `border-radius`, so an oversized one produces a pill.
pub fn rounded_rect(x: f32, y: f32, width: f32, height: f32, radius: f32) -> Vec<Pt> {
    let r = radius.min(width / 2.0).min(height / 2.0).max(0.0);
    let (right, bottom) = (x + width, y + height);
    let mut points = Vec::new();
    points.extend(arc(pt(right - r, y + r), r, -90.0, 0.0));
    points.extend(arc(pt(right - r, bottom - r), r, 0.0, 90.0));
    points.extend(arc(pt(x + r, bottom - r), r, 90.0, 180.0));
    points.extend(arc(pt(x + r, y + r), r, 180.0, 270.0));
    points.dedup();
    points
}

pub fn centered_rect(center: Pt, width: f32, height: f32, radius: f32) -> Vec<Pt> {
    rounded_rect(
        center.x - width / 2.0,
        center.y - height / 2.0,
        width,
        height,
        radius,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn rotation_is_clockwise_on_screen() {
        let p = Affine::rotate(90.0).apply(pt(1.0, 0.0));
        assert!(close(p.x, 0.0) && close(p.y, 1.0));
    }

    #[test]
    fn then_applies_in_order() {
        let m = Affine::translate(1.0, 0.0).then(Affine::linear(2.0, 0.0, 0.0, 2.0));
        assert_eq!(m.apply(pt(0.0, 0.0)), pt(2.0, 0.0));
    }

    #[test]
    fn about_keeps_the_origin_fixed() {
        let c = pt(12.0, 12.0);
        let p = Affine::rotate(123.0).about(c).apply(c);
        assert!(close(p.x, 12.0) && close(p.y, 12.0));
    }

    #[test]
    fn rounded_rect_clamps_its_radius() {
        let pill = rounded_rect(0.0, 0.0, 4.0, 1.0, 10.0);
        assert!(pill.iter().all(|p| p.y >= -1e-4 && p.y <= 1.0 + 1e-4));
        assert!(pill.iter().any(|p| close(p.x, 0.0)) && pill.iter().any(|p| close(p.x, 4.0)));
    }
}
