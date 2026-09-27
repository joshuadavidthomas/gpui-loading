use super::prelude::*;

/// A beam sweeping a scope, fading toward its edge.
#[derive(IntoElement)]
pub struct Radar {
    props: SpinnerProps,
    easing: Easing,
}

spinner_props!(Radar, easing);
easing_prop!(Radar);

impl RenderOnce for Radar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Radar { props, easing } = self;
        frame(
            props,
            SpinnerName::Radar,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                p.view(16.0);
                let center = pt(8.0, 8.0);
                p.circle(center, 8.0, 0.2);
                p.ring(center, 6.0, 1.0, 0.2);
                let turn = Affine::rotate(spin(m, easing)).about(center);
                p.sprite(Sprite::Asset(Asset::RadarBeam), turn, 1.0);
                p.circle(center, 2.0, 1.0);
            },
        )
    }
}
