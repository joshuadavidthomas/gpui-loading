use super::prelude::*;

const BARS: u16 = 12;

/// Twelve bars around a center, fading in turn — the classic system spinner.
#[derive(IntoElement)]
pub struct Classic(SpinnerProps);

spinner_props!(Classic);

impl RenderOnce for Classic {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Classic,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                let center = pt(0.5, 0.5);
                for index in 0..BARS {
                    let alpha = if m.reduced() {
                        0.5
                    } else {
                        1.0_f32.lerp(0.15, m.staggered(index.into(), BARS.into()))
                    };
                    let turn = Affine::rotate(f32::from(index) * 30.0).about(center);
                    p.sprite(Sprite::Asset(Asset::ClassicBar), turn, alpha);
                }
            },
        )
    }
}
