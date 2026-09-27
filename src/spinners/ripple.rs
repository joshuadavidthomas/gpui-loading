use super::prelude::*;

/// Whether the rings of [`Ripple`] spread outward or gather inward.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum RippleDirection {
    In,
    #[default]
    Out,
}

pub const DEFAULT_RIPPLE_DIRECTION: RippleDirection = RippleDirection::Out;

impl RippleDirection {
    pub const ALL: [RippleDirection; 2] = [RippleDirection::In, RippleDirection::Out];
}

const RINGS: u16 = 3;

const STROKE: f32 = 0.08;

/// Rings spreading from the center and fading.
#[derive(IntoElement)]
pub struct Ripple {
    props: SpinnerProps,
    direction: RippleDirection,
}

spinner_props!(Ripple, direction);

impl Ripple {
    #[must_use]
    pub fn direction(mut self, direction: RippleDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl RenderOnce for Ripple {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Ripple { props, direction } = self;
        frame(
            props,
            SpinnerName::Ripple,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                let center = pt(0.5, 0.5);
                for index in 0..RINGS {
                    let [scale, opacity] = if m.reduced() {
                        [f32::from(index + 1) / f32::from(RINGS), 0.4]
                    } else {
                        let progress = m.staggered(index.into(), RINGS.into());
                        let progress = match direction {
                            RippleDirection::Out => progress,
                            RippleDirection::In => 1.0 - progress,
                        };
                        keyframes(
                            progress,
                            &[(0.0, [0.0, 1.0]), (1.0, [1.0, 0.0])],
                            Timing::EASE_OUT,
                        )
                    };
                    p.ring(center, 0.5 * scale, STROKE * scale, opacity);
                }
            },
        )
    }
}
