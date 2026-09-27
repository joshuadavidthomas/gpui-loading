use super::prelude::*;

/// A head with a tail that fades out behind it.
#[derive(IntoElement)]
pub struct Comet {
    props: SpinnerProps,
    easing: Easing,
}

spinner_props!(Comet, easing);
easing_prop!(Comet);

impl RenderOnce for Comet {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Comet { props, easing } = self;
        frame(
            props,
            SpinnerName::Comet,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                let turn = Affine::rotate(spin(m, easing)).about(pt(0.5, 0.5));
                p.sprite(Sprite::Asset(Asset::Comet), turn, 1.0);
            },
        )
    }
}
