use super::prelude::*;

const DOTS: u16 = 3;

const DOT: f32 = 0.25;

const GAP: f32 = 0.2;

const WIDTH: f32 = DOT * DOTS as f32 + GAP * (DOTS - 1) as f32;

/// Three dots bouncing in turn.
#[derive(IntoElement)]
pub struct BouncingDots(SpinnerProps);

spinner_props!(BouncingDots);

impl RenderOnce for BouncingDots {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::BouncingDots,
            (WIDTH, 1.0),
            window,
            cx,
            |p, m| {
                for dot in 0..DOTS {
                    let lift = if m.reduced() {
                        0.0
                    } else {
                        0.28_f32.lerp(
                            -0.72,
                            Timing::EASE_IN_OUT.apply(m.alternating(dot.into(), DOTS.into())),
                        ) * DOT
                    };
                    let center = pt(f32::from(dot) * (DOT + GAP) + DOT / 2.0, 0.5 + lift);
                    p.circle(center, DOT / 2.0, 1.0);
                }
            },
        )
    }
}
