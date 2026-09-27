use super::prelude::*;

const BLOCK: [Pt; 4] = [pt(0.0, 0.0), pt(2.0, 0.0), pt(0.0, 2.0), pt(2.0, 2.0)];

const SEGMENTS: [Pt; 8] = [
    pt(12.0, 6.0),
    pt(10.0, 10.0),
    pt(6.0, 12.0),
    pt(2.0, 10.0),
    pt(0.0, 6.0),
    pt(2.0, 2.0),
    pt(6.0, 0.0),
    pt(10.0, 2.0),
];

/// Eight clusters of pixels around a center, fading in turn.
#[derive(IntoElement)]
pub struct Loading(SpinnerProps);

spinner_props!(Loading);

impl RenderOnce for Loading {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Loading,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                p.view(15.0);
                for (index, segment) in SEGMENTS.iter().enumerate() {
                    let alpha = if m.reduced() {
                        0.6
                    } else {
                        1.0_f32.lerp(0.2, m.staggered(index, SEGMENTS.len()))
                    };
                    for pixel in BLOCK {
                        p.rect(
                            segment.x + pixel.x,
                            segment.y + pixel.y,
                            1.0,
                            1.0,
                            0.0,
                            alpha,
                        );
                    }
                }
            },
        )
    }
}
