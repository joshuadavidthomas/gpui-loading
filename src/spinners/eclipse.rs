use super::prelude::*;

const DOTS: u16 = 2;

const DOT: f32 = 0.4;

/// Two dots passing in front of and behind each other.
#[derive(IntoElement)]
pub struct Eclipse(SpinnerProps);

spinner_props!(Eclipse);

impl RenderOnce for Eclipse {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Eclipse,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                for index in 0..DOTS {
                    let (slide, [opacity, scale]) = if m.reduced() {
                        (f32::from(index) * 1.5 - 0.75, [1.0, 1.0])
                    } else {
                        let progress = m.staggered(index.into(), DOTS.into());
                        let slide = keyframes(
                            progress,
                            &[(0.0, 0.75), (0.5, -0.75), (1.0, 0.75)],
                            Timing::EASE_IN_OUT,
                        );
                        let depth = keyframes(
                            progress,
                            &[
                                (0.0, [0.75, 1.0]),
                                (0.25, [1.0, 1.3]),
                                (0.5, [0.75, 1.0]),
                                (0.75, [0.5, 0.7]),
                                (1.0, [0.75, 1.0]),
                            ],
                            Timing::EASE_IN_OUT,
                        );
                        (slide, depth)
                    };
                    p.animated_circle(pt(0.5 + slide * DOT, 0.5), DOT / 2.0 * scale, opacity);
                }
            },
        )
    }
}
