use super::prelude::*;

const DOTS: u16 = 3;

const DOT: f32 = 0.1875;

const WIDTH: f32 = DOT * (DOTS * 2 - 1) as f32;

/// Three dots in a row, dimming in turn.
#[derive(IntoElement)]
pub struct LinearDots(SpinnerProps);

spinner_props!(LinearDots);

impl RenderOnce for LinearDots {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::LinearDots,
            (WIDTH, 1.0),
            window,
            cx,
            |p, m| {
                // Laid out in whole pixels, so small dots between pixels
                // don't blur out of round.
                let size = p.size();
                p.view(size);
                let dot_size = (size * DOT).round();
                let gap =
                    ((size * WIDTH - dot_size * f32::from(DOTS)) / f32::from(DOTS - 1)).floor();
                let span = dot_size * f32::from(DOTS) + gap * f32::from(DOTS - 1);
                let start = ((size * WIDTH - span) / 2.0).round();
                let top = ((size - dot_size) / 2.0).round();
                for dot in 0..DOTS {
                    let alpha = if m.reduced() {
                        0.75
                    } else {
                        keyframes(
                            m.staggered(dot.into(), DOTS.into()),
                            &[(0.0, 1.0), (0.6667, 0.5), (1.0, 1.0)],
                            Timing::Linear,
                        )
                    };
                    p.circle(
                        pt(
                            start + f32::from(dot) * (dot_size + gap) + dot_size / 2.0,
                            top + dot_size / 2.0,
                        ),
                        dot_size / 2.0,
                        alpha,
                    );
                }
            },
        )
    }
}
