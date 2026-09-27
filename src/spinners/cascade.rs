use super::prelude::*;

const RADII: [f32; 3] = [10.5, 7.0, 3.5];

const SLOTS: usize = 24;

const TURN: Timing = Timing::CubicBezier(0.68, -0.75, 0.265, 1.75);

/// Three nested quarter arcs that whip around one after another.
#[derive(IntoElement)]
pub struct Cascade {
    props: SpinnerProps,
    cap: Cap,
}

spinner_props!(Cascade, cap);
cap_prop!(Cascade);

impl RenderOnce for Cascade {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Cascade { props, cap } = self;
        frame(
            props,
            SpinnerName::Cascade,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(24.0);
                for (index, radius) in RADII.into_iter().enumerate() {
                    let angle = if m.reduced() {
                        0.0
                    } else {
                        360.0 * TURN.apply(m.staggered(index, SLOTS))
                    };
                    let quarter = std::f32::consts::TAU * radius / 4.0;
                    let turn = Affine::rotate(angle).about(pt(12.0, 12.0));
                    p.sprite(arc(24.0, radius, 2.0, quarter, cap), turn, 1.0);
                }
            },
        )
    }
}
