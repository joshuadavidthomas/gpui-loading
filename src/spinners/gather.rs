use super::prelude::*;

/// The pull is rounded to this, bounding how many sprites it needs.
const QUANTUM: f32 = 0.02;

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
                let blocks = Sprite::GatherBlocks {
                    pull: milli((pull / QUANTUM).round() * QUANTUM),
                };
                p.sprite(blocks, Affine::rotate(turn).about(pt(0.5, 0.5)), 1.0);
            },
        )
    }
}
