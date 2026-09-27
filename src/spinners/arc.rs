use super::prelude::*;

/// A single arc turning around a circle.
#[derive(IntoElement)]
pub struct Arc {
    props: SpinnerProps,
    easing: Easing,
    cap: Cap,
}

spinner_props!(Arc, easing, cap);
easing_prop!(Arc);
cap_prop!(Arc);

impl RenderOnce for Arc {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Arc { props, easing, cap } = self;
        frame(
            props,
            SpinnerName::Arc,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(24.0);
                let turn = Affine::rotate(spin(m, easing)).about(pt(12.0, 12.0));
                p.sprite(arc(24.0, 10.0, 2.5, 18.0, cap), turn, 1.0);
            },
        )
    }
}
