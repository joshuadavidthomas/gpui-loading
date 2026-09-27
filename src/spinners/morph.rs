use super::prelude::*;

const SHAPE: f32 = 0.64;

/// The corner radius is rounded to this, bounding how many sprites the
/// morph needs.
const RADIUS_QUANTUM: f32 = 0.005;

/// A square rounding into a circle and back as it turns.
#[derive(IntoElement)]
pub struct Morph(SpinnerProps);

spinner_props!(Morph);

impl RenderOnce for Morph {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Morph,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                let [radius, turn] = if m.reduced() {
                    [0.08, 0.0]
                } else {
                    keyframes(
                        m.progress(),
                        &[
                            (0.0, [0.08, 0.0]),
                            (0.5, [SHAPE / 2.0, 45.0]),
                            (1.0, [0.08, 90.0]),
                        ],
                        Timing::EASE_IN_OUT,
                    )
                };
                let shape = Sprite::RoundedRect {
                    view: milli(1.0),
                    width: milli(SHAPE),
                    height: milli(SHAPE),
                    radius: milli((radius / RADIUS_QUANTUM).round() * RADIUS_QUANTUM),
                };
                p.sprite(shape, Affine::rotate(turn).about(pt(0.5, 0.5)), 1.0);
            },
        )
    }
}
