use super::prelude::*;

/// A dot sending out a fading ring.
#[derive(IntoElement)]
pub struct Pulse(SpinnerProps);

spinner_props!(Pulse);

impl RenderOnce for Pulse {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Pulse,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                p.view(16.0);
                let center = pt(8.0, 8.0);
                let [scale, opacity] = if m.reduced() {
                    [1.0, 0.2]
                } else {
                    keyframes(
                        m.progress(),
                        &[(0.0, [0.25, 0.4]), (1.0, [1.0, 0.0])],
                        Timing::EASE_OUT,
                    )
                };
                p.circle(center, 8.0 * scale, opacity);
                p.dot(center, 2.0, 1.0);
            },
        )
    }
}
