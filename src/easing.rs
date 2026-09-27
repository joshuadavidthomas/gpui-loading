//! Timing functions and keyframe interpolation — the parts of CSS animations
//! the spinners rely on.

/// How a rotating spinner moves through each turn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Easing {
    /// A constant speed.
    #[default]
    Linear,
    /// Slow at the start and end of every turn.
    EaseInOut,
    /// A linear and an ease-in-out turn added together: two turns per
    /// cycle that surge in the middle.
    Stacked,
}

pub const DEFAULT_EASING: Easing = Easing::Linear;

impl Easing {
    pub const ALL: [Easing; 3] = [Easing::Linear, Easing::EaseInOut, Easing::Stacked];

    /// How many turns the spinner has made at `progress` through a cycle.
    pub(crate) fn turns(self, progress: f32) -> f32 {
        match self {
            Easing::Linear => progress,
            Easing::EaseInOut => Timing::EASE_IN_OUT.apply(progress),
            Easing::Stacked => progress + Timing::EASE_IN_OUT.apply(progress),
        }
    }
}

/// A CSS `<easing-function>`: `linear` or a `cubic-bezier()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Timing {
    Linear,
    CubicBezier(f32, f32, f32, f32),
}

impl Timing {
    pub const EASE_IN_OUT: Timing = Timing::CubicBezier(0.42, 0.0, 0.58, 1.0);
    pub const EASE_OUT: Timing = Timing::CubicBezier(0.0, 0.0, 0.58, 1.0);

    pub fn apply(self, t: f32) -> f32 {
        match self {
            Timing::Linear => t,
            Timing::CubicBezier(x1, y1, x2, y2) => cubic_bezier(x1, y1, x2, y2, t),
        }
    }
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "the curve is solved in f64 for precision and returned as the f32 it was given"
)]
fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let (x1, y1, x2, y2, x) = (
        f64::from(x1),
        f64::from(y1),
        f64::from(x2),
        f64::from(y2),
        f64::from(x),
    );
    let sample = |a: f64, b: f64, t: f64| {
        let u = 1.0 - t;
        3.0 * u * u * t * a + 3.0 * u * t * t * b + t * t * t
    };
    let slope = |a: f64, b: f64, t: f64| {
        let u = 1.0 - t;
        3.0 * u * u * a + 6.0 * u * t * (b - a) + 3.0 * t * t * (1.0 - b)
    };

    // Newton's method, falling back to bisection where the slope flattens.
    let mut t = x;
    for _ in 0..8 {
        let error = sample(x1, x2, t) - x;
        if error.abs() < 1e-7 {
            return sample(y1, y2, t) as f32;
        }
        let d = slope(x1, x2, t);
        if d.abs() < 1e-6 {
            break;
        }
        t -= error / d;
    }
    let (mut lo, mut hi) = (0.0, 1.0);
    t = x;
    for _ in 0..40 {
        let value = sample(x1, x2, t);
        if (value - x).abs() < 1e-7 {
            break;
        }
        if value < x {
            lo = t;
        } else {
            hi = t;
        }
        t = f64::midpoint(lo, hi);
    }
    sample(y1, y2, t) as f32
}

pub(crate) trait Lerp: Copy {
    fn lerp(self, to: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, to: f32, t: f32) -> f32 {
        self + (to - self) * t
    }
}

impl<const N: usize> Lerp for [f32; N] {
    fn lerp(self, to: Self, t: f32) -> Self {
        std::array::from_fn(|i| self[i].lerp(to[i], t))
    }
}

/// The value of a `@keyframes` rule at `progress`, with `timing` applied
/// to each interval between keyframes, as CSS does. `frames` are
/// `(offset, value)` pairs sorted by offset, spanning `0.0..=1.0`.
pub(crate) fn keyframes<T: Lerp>(progress: f32, frames: &[(f32, T)], timing: Timing) -> T {
    let i = frames
        .iter()
        .rposition(|&(offset, _)| offset <= progress)
        .unwrap_or(0)
        .min(frames.len() - 2);
    let ((from_at, from), (to_at, to)) = (frames[i], frames[i + 1]);
    let local = if to_at > from_at {
        ((progress - from_at) / (to_at - from_at)).clamp(0.0, 1.0)
    } else {
        1.0
    };
    from.lerp(to, timing.apply(local))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn ease_in_out_matches_css() {
        let ease = Timing::EASE_IN_OUT;
        assert!(close(ease.apply(0.0), 0.0));
        assert!(close(ease.apply(0.5), 0.5));
        assert!(close(ease.apply(1.0), 1.0));
        assert!(close(ease.apply(0.25), 0.1291));
    }

    #[test]
    fn cubic_bezier_can_overshoot() {
        let back = Timing::CubicBezier(0.68, -0.75, 0.265, 1.75);
        let values: Vec<f32> = (1_u8..100)
            .map(|i| back.apply(f32::from(i) / 100.0))
            .collect();
        assert!(values.iter().any(|&v| v < 0.0));
        assert!(values.iter().any(|&v| v > 1.0));
    }

    #[test]
    fn keyframes_ease_each_interval() {
        let frames = [(0.0, 1.0), (0.35, 0.0), (0.7, 1.0), (1.0, 1.0)];
        assert!(close(keyframes(0.0, &frames, Timing::EASE_IN_OUT), 1.0));
        assert!(close(keyframes(0.175, &frames, Timing::EASE_IN_OUT), 0.5));
        assert!(close(keyframes(0.35, &frames, Timing::EASE_IN_OUT), 0.0));
        assert!(close(keyframes(0.85, &frames, Timing::EASE_IN_OUT), 1.0));
    }

    #[test]
    fn stacked_easing_makes_two_turns() {
        assert!(close(Easing::Stacked.turns(1.0), 2.0));
        assert!(close(Easing::Stacked.turns(0.5), 1.0));
    }
}
