use super::prelude::*;

/// The order the cells of [`Blocks`] shrink in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BlocksSweep {
    Columns,
    #[default]
    Diagonal,
    Rows,
}

pub const DEFAULT_BLOCKS_SWEEP: BlocksSweep = BlocksSweep::Diagonal;

impl BlocksSweep {
    pub const ALL: [BlocksSweep; 3] = [
        BlocksSweep::Columns,
        BlocksSweep::Diagonal,
        BlocksSweep::Rows,
    ];

    /// How many steps the sweep takes, and which one a cell is on.
    fn place(self, col: u16, row: u16) -> (u16, u16) {
        match self {
            BlocksSweep::Columns => (SIDE, col),
            BlocksSweep::Diagonal => (SIDE * 2 - 1, row + col),
            BlocksSweep::Rows => (SIDE, row),
        }
    }
}

const SIDE: u16 = 3;

const GAP: f32 = 0.1;

const CELL: f32 = (1.0 - GAP * (SIDE - 1) as f32) / SIDE as f32;

/// A 3×3 grid whose cells shrink and grow in a sweep.
#[derive(IntoElement)]
pub struct Blocks {
    props: SpinnerProps,
    sweep: BlocksSweep,
}

spinner_props!(Blocks, sweep);

impl Blocks {
    #[must_use]
    pub fn sweep(mut self, sweep: BlocksSweep) -> Self {
        self.sweep = sweep;
        self
    }
}

impl RenderOnce for Blocks {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Blocks { props, sweep } = self;
        frame(
            props,
            SpinnerName::Blocks,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                for row in 0..SIDE {
                    for col in 0..SIDE {
                        let (count, place) = sweep.place(col, row);
                        let scale = if m.reduced() {
                            0.8
                        } else {
                            keyframes(
                                m.staggered(place.into(), count.into()),
                                &[(0.0, 1.0), (0.35, 0.0), (0.7, 1.0), (1.0, 1.0)],
                                Timing::EASE_IN_OUT,
                            )
                        };
                        let side = CELL * scale;
                        let x = f32::from(col) * (CELL + GAP) + (CELL - side) / 2.0;
                        let y = f32::from(row) * (CELL + GAP) + (CELL - side) / 2.0;
                        p.rect(x, y, side, side, 0.0625 * scale, 1.0);
                    }
                }
            },
        )
    }
}
