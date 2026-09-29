use super::prelude::*;
use crate::sprite::GATHER_BLOCK;
use crate::sprite::GATHER_BLOCKS;
use crate::sprite::GATHER_PULL;

/// Four blocks drawing together, turning a quarter, and parting.
#[derive(IntoElement)]
pub struct Gather(SpinnerProps);

spinner_props!(Gather);

impl RenderOnce for Gather {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Gather,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                let (turn, pull) = if m.reduced() {
                    (0.0, 0.0)
                } else {
                    let progress = m.progress();
                    (
                        keyframes(
                            progress,
                            &[(0.0, 0.0), (0.3, 0.0), (0.6, 90.0), (1.0, 90.0)],
                            Timing::EASE_IN_OUT,
                        ),
                        keyframes(
                            progress,
                            &[(0.0, 0.0), (0.3, 1.0), (0.6, 1.0), (1.0, 0.0)],
                            Timing::EASE_IN_OUT,
                        ),
                    )
                };
                for (corner, direction) in GATHER_BLOCKS {
                    let shift = GATHER_PULL * pull;
                    let center =
                        corner + direction * shift + pt(GATHER_BLOCK / 2.0, GATHER_BLOCK / 2.0);
                    let transform = Affine::translate(center.x - 0.5, center.y - 0.5)
                        .then(Affine::rotate(turn).about(pt(0.5, 0.5)));
                    p.sprite(Sprite::GatherBlock, transform, 1.0);
                }
            },
        )
    }
}
