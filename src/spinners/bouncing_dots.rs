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
                // Laid out across in whole pixels, so small dots between
                // pixels don't antialias lopsided.
                let size = p.size();
                p.view(size);
                let dot_size = (size * DOT).round();
                let gap = (size * GAP).round();
                let span = dot_size * f32::from(DOTS) + gap * f32::from(DOTS - 1);
                let start = ((size * WIDTH - span) / 2.0).round();
                for dot in 0..DOTS {
                    let lift = if m.reduced() {
                        0.0
                    } else {
                        0.28_f32.lerp(
                            -0.72,
                            Timing::EASE_IN_OUT.apply(m.alternating(dot.into(), DOTS.into())),
                        ) * dot_size
                    };
                    let center = pt(
                        start + f32::from(dot) * (dot_size + gap) + dot_size / 2.0,
                        size / 2.0 + lift,
                    );
                    p.circle(center, dot_size / 2.0, 1.0);
                }
            },
        )
    }
}
