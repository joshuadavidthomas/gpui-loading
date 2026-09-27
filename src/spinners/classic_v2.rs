use super::prelude::*;

const TICKS: u16 = 8;

/// Eight rounded ticks around a center, fading in turn.
#[derive(IntoElement)]
pub struct ClassicV2(SpinnerProps);

spinner_props!(ClassicV2);

impl RenderOnce for ClassicV2 {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::ClassicV2,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                p.view(16.0);
                for index in 0..TICKS {
                    let alpha = if m.reduced() {
                        0.5
                    } else {
                        1.0_f32.lerp(0.4, m.staggered(index.into(), TICKS.into()))
                    };
                    let turn = Affine::rotate(f32::from(index) * 45.0).about(pt(8.0, 8.0));
                    p.sprite(Sprite::Asset(Asset::Tick), turn, alpha);
                }
            },
        )
    }
}
