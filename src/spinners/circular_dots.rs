use super::prelude::*;

const DOTS: [Pt; 8] = [
    pt(8.0, 1.5),
    pt(12.5962, 3.4038),
    pt(14.5, 8.0),
    pt(12.5962, 12.5962),
    pt(8.0, 14.5),
    pt(3.4038, 12.5962),
    pt(1.5, 8.0),
    pt(3.4038, 3.4038),
];

/// Eight dots in a circle, fading in turn.
#[derive(IntoElement)]
pub struct CircularDots(SpinnerProps);

spinner_props!(CircularDots);

impl RenderOnce for CircularDots {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::CircularDots,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                p.view(16.0);
                for (index, &center) in DOTS.iter().enumerate() {
                    let alpha = if m.reduced() {
                        0.6
                    } else {
                        1.0.lerp(0.2, m.staggered(index, DOTS.len()))
                    };
                    p.dot(center, 1.5, alpha);
                }
            },
        )
    }
}
