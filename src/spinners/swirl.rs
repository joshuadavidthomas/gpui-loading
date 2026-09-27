use super::prelude::*;

/// Each cell's place in the swirl, row by row; the center is empty.
const RING: [Option<usize>; 9] = [
    Some(0),
    Some(1),
    Some(2),
    Some(7),
    None,
    Some(3),
    Some(6),
    Some(5),
    Some(4),
];

const PLACES: usize = 8;

const PADDING: f32 = 0.0625;

const GAP: f32 = 0.15625;

const CELL: f32 = (1.0 - 2.0 * PADDING - 2.0 * GAP) / 3.0;

/// Eight squares around an empty center, fading in turn.
#[derive(IntoElement)]
pub struct Swirl(SpinnerProps);

spinner_props!(Swirl);

impl RenderOnce for Swirl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Swirl,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                for (index, place) in (0_u16..).zip(RING) {
                    let Some(place) = place else { continue };
                    let alpha = if m.reduced() {
                        0.6
                    } else {
                        1.0_f32.lerp(0.2, m.staggered(place, PLACES))
                    };
                    let (col, row) = (f32::from(index % 3), f32::from(index / 3));
                    let (x, y) = (PADDING + col * (CELL + GAP), PADDING + row * (CELL + GAP));
                    p.rect(x, y, CELL, CELL, 0.0625, alpha);
                }
            },
        )
    }
}
