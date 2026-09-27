use super::prelude::*;

const ORBITS: u16 = 3;

const TILT: f32 = 180.0 / ORBITS as f32;

const STROKE: f32 = 0.055;

/// The squash is rounded to this, bounding how many sprites it needs.
const QUANTUM: f32 = 0.01;

/// A shell with three orbits tumbling in 3D.
#[derive(IntoElement)]
pub struct Atom {
    props: SpinnerProps,
    easing: Easing,
}

spinner_props!(Atom, easing);
easing_prop!(Atom);

impl RenderOnce for Atom {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Atom { props, easing } = self;
        frame(
            props,
            SpinnerName::Atom,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                let center = pt(0.5, 0.5);
                p.ring(center, 0.5, STROKE, 1.0);
                for index in 0..ORBITS {
                    let spin = if m.reduced() {
                        TILT
                    } else {
                        360.0 * easing.turns(m.staggered(index.into(), ORBITS.into()))
                    };
                    // rotate(tilt) rotateX(90deg) rotate(spin) rotateX(90deg),
                    // seen without perspective: a circle squashed to cos(spin)
                    // across, then tilted.
                    let squash = (spin.to_radians().cos().abs() / QUANTUM).round() * QUANTUM;
                    let tilt = Affine::rotate(f32::from(index) * TILT).about(center);
                    p.sprite(
                        Sprite::AtomOrbit {
                            squash: milli(squash),
                        },
                        tilt,
                        1.0,
                    );
                }
            },
        )
    }
}
