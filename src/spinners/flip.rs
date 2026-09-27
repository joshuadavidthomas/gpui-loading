use super::prelude::*;

/// A square flipping over one axis, then the other.
#[derive(IntoElement)]
pub struct Flip(SpinnerProps);

spinner_props!(Flip);

/// The face turned `degrees` (between -180 and 0) about its horizontal
/// axis. Turns past a quarter mirror turns short of one, so the sprites only
/// cover 0 to -90, a degree apart.
fn face(degrees: f32) -> (Sprite, Affine) {
    let center = pt(0.5, 0.5);
    if degrees < -90.0 {
        let (sprite, _) = face(-180.0 - degrees);
        (sprite, Affine::linear(1.0, 0.0, 0.0, -1.0).about(center))
    } else {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "the face is only ever turned between -90 and 0 degrees"
        )]
        let degrees = degrees.round() as i32;
        (Sprite::FlipFace { degrees }, Affine::IDENTITY)
    }
}

impl RenderOnce for Flip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        frame(self.0, SpinnerName::Flip, (1.0, 1.0), window, cx, |p, m| {
            let [x, y] = if m.reduced() {
                [0.0, 0.0]
            } else {
                keyframes(
                    m.progress(),
                    &[
                        (0.0, [0.0, 0.0]),
                        (0.5, [-180.0, 0.0]),
                        (1.0, [-180.0, -180.0]),
                    ],
                    Timing::EASE_IN_OUT,
                )
            };
            // rotateX(-180deg) rotateY(y) looks like rotateX(y) turned a
            // quarter counterclockwise, the face being square.
            let (sprite, affine) = if y == 0.0 {
                face(x)
            } else {
                let (sprite, affine) = face(y);
                (
                    sprite,
                    affine.then(Affine::rotate(-90.0).about(pt(0.5, 0.5))),
                )
            };
            p.sprite(sprite, affine, 1.0);
        })
    }
}
