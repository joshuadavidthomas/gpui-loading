use super::prelude::*;

const DOT: f32 = 0.34;

const GAP: f32 = 0.2;

const MARGIN: f32 = (1.0 - 2.0 * DOT - GAP) / 2.0;

/// One step across, as the web version rounds it: 158.8% of a dot.
const FAR: f32 = 1.588 * DOT;

/// Where each dot rests under reduced motion.
const RESTS: [[f32; 2]; 3] = [[FAR, 0.0], [0.0, 0.0], [0.0, FAR]];

const WALK: [(f32, [f32; 2]); 9] = [
    (0.0, [FAR, 0.0]),
    (0.0833, [FAR, FAR]),
    (0.25, [FAR, FAR]),
    (0.3333, [0.0, FAR]),
    (0.5, [0.0, FAR]),
    (0.5833, [0.0, 0.0]),
    (0.75, [0.0, 0.0]),
    (0.8333, [FAR, 0.0]),
    (1.0, [FAR, 0.0]),
];

/// Three dots taking turns sliding around a square.
#[derive(IntoElement)]
pub struct Slide(SpinnerProps);

spinner_props!(Slide);

impl RenderOnce for Slide {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Slide,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                for (index, rest) in RESTS.into_iter().enumerate() {
                    let [x, y] = if m.reduced() {
                        rest
                    } else {
                        keyframes(m.staggered(index, RESTS.len()), &WALK, Timing::EASE_IN_OUT)
                    };
                    let center = pt(MARGIN + DOT / 2.0 + x, MARGIN + DOT / 2.0 + y);
                    p.circle(center, DOT / 2.0, 1.0);
                }
            },
        )
    }
}
