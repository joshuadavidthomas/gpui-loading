use super::prelude::*;

const DOTS: u16 = 3;

/// Dots leapfrogging over each other.
#[derive(IntoElement)]
pub struct Leap(SpinnerProps);

spinner_props!(Leap);

impl RenderOnce for Leap {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(self.0, SpinnerName::Leap, (1.0, 1.0), window, cx, |p, m| {
            // Laid out in whole pixels, as the web version rounds them.
            let size = p.size();
            p.view(size);
            let dot = (size * 0.22).round();
            let gap = ((size - dot) / 2.0).floor();
            let top = ((size - dot) / 2.0).round();
            let left = size - dot - gap * 2.0;
            let hinge = pt(left + gap + dot / 2.0, top + dot / 2.0);
            for index in 0..DOTS {
                let [shift, turn] = if m.reduced() {
                    [f32::from(index), 0.0]
                } else {
                    keyframes(
                        m.staggered(index.into(), DOTS.into()),
                        &[
                            (0.0, [0.0, 0.0]),
                            (0.3333, [0.0, 180.0]),
                            (0.6666, [-1.0, 180.0]),
                            (1.0, [-2.0, 180.0]),
                        ],
                        Timing::EASE_IN_OUT,
                    )
                };
                let arm = Affine::rotate(turn).apply(pt(-gap, 0.0));
                let center = pt(hinge.x + shift * gap + arm.x, hinge.y + arm.y);
                p.circle(center, dot / 2.0, 1.0);
            }
        })
    }
}
