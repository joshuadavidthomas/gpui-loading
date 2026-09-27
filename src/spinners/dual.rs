use super::prelude::*;

/// Two arcs turning in opposite directions.
#[derive(IntoElement)]
pub struct Dual {
    props: SpinnerProps,
    easing: Easing,
    cap: Cap,
}

spinner_props!(Dual, easing, cap);
easing_prop!(Dual);
cap_prop!(Dual);

impl RenderOnce for Dual {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Dual { props, easing, cap } = self;
        frame(
            props,
            SpinnerName::Dual,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(24.0);
                let center = pt(12.0, 12.0);
                let inner = if m.reduced() {
                    180.0
                } else {
                    // animation-direction: reverse
                    360.0 * easing.turns(1.0 - m.progress())
                };
                let outer = Affine::rotate(spin(m, easing)).about(center);
                p.sprite(arc(24.0, 10.0, 2.5, 18.0, cap), outer, 1.0);
                let inner = Affine::rotate(inner).about(center);
                p.sprite(arc(24.0, 5.5, 2.5, 10.0, cap), inner, 1.0);
            },
        )
    }
}
