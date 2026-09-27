use super::prelude::*;
use crate::sprite::TRACE_PERIMETER;
use crate::sprite::TRACE_RADIUS;
use crate::sprite::TRACE_SIDE;
use crate::sprite::TRACE_STROKE;
use crate::sprite::TRACE_VIEW;

/// Dash offsets are rounded to this, bounding how many sprites the trace
/// needs.
const QUANTUM: f32 = 0.125;

/// A dash running around a rounded square.
#[derive(IntoElement)]
pub struct Trace {
    props: SpinnerProps,
    easing: Easing,
    cap: Cap,
}

spinner_props!(Trace, easing, cap);
easing_prop!(Trace);
cap_prop!(Trace);

impl RenderOnce for Trace {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Trace { props, easing, cap } = self;
        frame(
            props,
            SpinnerName::Trace,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(TRACE_VIEW);
                let center = pt(TRACE_VIEW / 2.0, TRACE_VIEW / 2.0);
                let outer = TRACE_SIDE + TRACE_STROKE;
                let radius = TRACE_RADIUS + TRACE_STROKE / 2.0;
                p.outline(center, outer, radius, TRACE_STROKE, 0.2);

                // The shared rotation, applied to the dash offset instead. The
                // square looks the same a quarter turn round, so a dash a
                // quarter further along is the same sprite turned 90°.
                let along = if m.reduced() {
                    0.0
                } else {
                    (TRACE_PERIMETER * easing.turns(m.progress())).rem_euclid(TRACE_PERIMETER)
                };
                let quarter = TRACE_PERIMETER / 4.0;
                let turns = (along / quarter).floor();
                let offset = ((along - turns * quarter) / QUANTUM).round() * QUANTUM;
                let turn = Affine::rotate(turns * 90.0).about(center);
                p.sprite(
                    Sprite::TraceDash {
                        offset: milli(offset),
                        cap,
                    },
                    turn,
                    1.0,
                );
            },
        )
    }
}
