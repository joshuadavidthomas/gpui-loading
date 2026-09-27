use super::prelude::*;

/// A pie slice sweeping around a faint dial.
#[derive(IntoElement)]
pub struct Clock {
    props: SpinnerProps,
    easing: Easing,
}

spinner_props!(Clock, easing);
easing_prop!(Clock);

impl RenderOnce for Clock {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Clock { props, easing } = self;
        frame(
            props,
            SpinnerName::Clock,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(16.0);
                let center = pt(8.0, 8.0);
                p.circle(center, 8.0, 0.1);
                let turn = Affine::rotate(spin(m, easing)).about(center);
                p.sprite(Sprite::Asset(Asset::ClockHand), turn, 1.0);
            },
        )
    }
}
