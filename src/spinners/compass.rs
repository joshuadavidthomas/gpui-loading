use super::prelude::*;

/// Four ticks snapping a quarter turn at a time.
#[derive(IntoElement)]
pub struct Compass(SpinnerProps);

spinner_props!(Compass);

impl RenderOnce for Compass {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Compass,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                p.view(16.0);
                let angle = if m.reduced() {
                    0.0
                } else {
                    keyframes(
                        m.progress(),
                        &[(0.0, 0.0), (1.0, 90.0)],
                        Timing::EASE_IN_OUT,
                    )
                };
                for quarter in 0_u8..4 {
                    let turn =
                        Affine::rotate(angle + f32::from(quarter) * 90.0).about(pt(8.0, 8.0));
                    p.sprite(Sprite::Asset(Asset::Tick), turn, 1.0);
                }
            },
        )
    }
}
