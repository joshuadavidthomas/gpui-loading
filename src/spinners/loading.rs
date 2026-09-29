use super::prelude::*;

/// Cells across the grid the segments are drawn on.
const GRID: f32 = 15.0;

const BLOCK: [Pt; 4] = [pt(0.0, 0.0), pt(2.0, 0.0), pt(0.0, 2.0), pt(2.0, 2.0)];

const SEGMENTS: [Pt; 8] = [
    pt(12.0, 6.0),
    pt(10.0, 10.0),
    pt(6.0, 12.0),
    pt(2.0, 10.0),
    pt(0.0, 6.0),
    pt(2.0, 2.0),
    pt(6.0, 0.0),
    pt(10.0, 2.0),
];

/// Eight clusters of pixels around a center, fading in turn.
#[derive(IntoElement)]
pub struct Loading(SpinnerProps);

spinner_props!(Loading);

impl RenderOnce for Loading {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(
            self.0,
            SpinnerName::Loading,
            (1.0, 1.0),
            window,
            cx,
            |p, m| {
                // Whole device pixels to a cell, centered: cells between
                // pixels smear unevenly at small sizes.
                let pixels = p.device_size();
                p.view(pixels);
                let cell = (pixels / GRID).floor().max(1.0);
                let offset = ((pixels - GRID * cell) / 2.0).floor();
                for (index, segment) in SEGMENTS.iter().enumerate() {
                    let alpha = if m.reduced() {
                        0.6
                    } else {
                        1.0_f32.lerp(0.2, m.staggered(index, SEGMENTS.len()))
                    };
                    for pixel in BLOCK {
                        p.rect(
                            offset + (segment.x + pixel.x) * cell,
                            offset + (segment.y + pixel.y) * cell,
                            cell,
                            cell,
                            0.0,
                            alpha,
                        );
                    }
                }
            },
        )
    }
}
