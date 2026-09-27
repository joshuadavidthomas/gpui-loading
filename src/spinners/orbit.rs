use super::prelude::*;

/// A dot circled by a half ring that fades out along its length.
#[derive(IntoElement)]
pub struct Orbit {
    props: SpinnerProps,
    easing: Easing,
}

spinner_props!(Orbit, easing);
easing_prop!(Orbit);

impl RenderOnce for Orbit {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Orbit { props, easing } = self;
        frame(
            props,
            SpinnerName::Orbit,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                let center = pt(0.5, 0.5);
                p.circle(center, 0.125, 1.0);
                let turn = Affine::rotate(spin(m, easing)).about(center);
                p.sprite(Sprite::Asset(Asset::Orbit), turn, 1.0);
            },
        )
    }
}
