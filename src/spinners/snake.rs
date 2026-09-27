use super::prelude::*;

const RADIUS: f32 = 10.0;

/// Dash lengths are rounded to this, bounding how many sprites the stretch
/// needs.
const QUANTUM: f32 = 0.25;

/// An arc stretching and shrinking as it turns.
#[derive(IntoElement)]
pub struct Snake {
    props: SpinnerProps,
    easing: Easing,
    cap: Cap,
}

spinner_props!(Snake, easing, cap);
easing_prop!(Snake);
cap_prop!(Snake);

impl RenderOnce for Snake {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Snake { props, easing, cap } = self;
        frame(
            props,
            SpinnerName::Snake,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(24.0);
                let [dash, offset] = if m.reduced() {
                    [18.0, 0.0]
                } else {
                    keyframes(
                        m.progress(),
                        &[
                            (0.0, [1.0, 0.0]),
                            (0.5, [45.0, -17.0]),
                            (1.0, [45.0, -62.0]),
                        ],
                        Timing::EASE_IN_OUT,
                    )
                };
                // stroke-dasharray: dash 100 with a negative offset paints one
                // dash starting `-offset` along the circle, cut off where the
                // circle's path ends: an arc, turned to where it starts.
                let circumference = std::f32::consts::TAU * RADIUS;
                let start = -offset;
                let length = (dash.min(circumference - start) / QUANTUM).round() * QUANTUM;
                if length > 0.0 {
                    let angle = spin(m, easing) + (start / RADIUS).to_degrees();
                    let turn = Affine::rotate(angle).about(pt(12.0, 12.0));
                    p.sprite(arc(24.0, RADIUS, 2.5, length, cap), turn, 1.0);
                }
            },
        )
    }
}
