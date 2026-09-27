use super::prelude::*;

/// An arc turning around a faint track.
#[derive(IntoElement)]
pub struct Ring {
    props: SpinnerProps,
    easing: Easing,
    cap: Cap,
}

spinner_props!(Ring, easing, cap);
easing_prop!(Ring);
cap_prop!(Ring);

impl RenderOnce for Ring {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Ring { props, easing, cap } = self;
        frame(
            props,
            SpinnerName::Ring,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(24.0);
                let center = pt(12.0, 12.0);
                p.ring(center, 11.25, 2.5, 0.2);
                let turn = Affine::rotate(spin(m, easing)).about(center);
                p.sprite(arc(24.0, 10.0, 2.5, 16.0, cap), turn, 1.0);
            },
        )
    }
}
