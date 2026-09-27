//! One module per spinner, each a port of its `src/<name>.tsx` counterpart.

mod arc;
mod atom;
mod blocks;
mod bouncing_dots;
mod cascade;
mod circular_dots;
mod classic;
mod classic_v2;
mod clock;
mod comet;
mod compass;
mod dual;
mod eclipse;
mod flip;
mod gather;
mod leap;
mod linear_dots;
mod loading;
mod morph;
mod orbit;
mod pulse;
mod radar;
mod ring;
mod ripple;
mod slide;
mod snake;
mod swirl;
mod trace;
mod wave;

pub use arc::Arc;
pub use atom::Atom;
pub use blocks::Blocks;
pub use blocks::BlocksSweep;
pub use blocks::DEFAULT_BLOCKS_SWEEP;
pub use bouncing_dots::BouncingDots;
pub use cascade::Cascade;
pub use circular_dots::CircularDots;
pub use classic::Classic;
pub use classic_v2::ClassicV2;
pub use clock::Clock;
pub use comet::Comet;
pub use compass::Compass;
pub use dual::Dual;
pub use eclipse::Eclipse;
pub use flip::Flip;
pub use gather::Gather;
pub use leap::Leap;
pub use linear_dots::LinearDots;
pub use loading::Loading;
pub use morph::Morph;
pub use orbit::Orbit;
pub use pulse::Pulse;
pub use radar::Radar;
pub use ring::Ring;
pub use ripple::DEFAULT_RIPPLE_DIRECTION;
pub use ripple::Ripple;
pub use ripple::RippleDirection;
pub use slide::Slide;
pub use snake::Snake;
pub use swirl::Swirl;
pub use trace::Trace;
pub use wave::DEFAULT_WAVE_ORIGIN;
pub use wave::Wave;
pub use wave::WaveOrigin;

/// What every spinner module needs.
mod prelude {
    pub(crate) use gpui::App;
    pub(crate) use gpui::IntoElement;
    pub(crate) use gpui::RenderOnce;
    pub(crate) use gpui::Window;

    pub(crate) use super::arc;
    pub(crate) use super::spin;
    pub(crate) use crate::cap::Cap;
    pub(crate) use crate::easing::Easing;
    pub(crate) use crate::easing::Lerp;
    pub(crate) use crate::easing::Timing;
    pub(crate) use crate::easing::keyframes;
    pub(crate) use crate::frame::SpinnerProps;
    pub(crate) use crate::frame::cap_prop;
    pub(crate) use crate::frame::easing_prop;
    pub(crate) use crate::frame::frame;
    pub(crate) use crate::frame::spinner_props;
    pub(crate) use crate::geom::Affine;
    pub(crate) use crate::geom::Pt;
    pub(crate) use crate::geom::pt;
    pub(crate) use crate::motion::SpinnerName;
    pub(crate) use crate::sprite::Asset;
    pub(crate) use crate::sprite::Sprite;
    pub(crate) use crate::sprite::milli;
}

use crate::cap::Cap;
use crate::easing::Easing;
use crate::motion::Motion;
use crate::sprite::Sprite;
use crate::sprite::milli;

/// The angle, in degrees, of a spinner using the shared rotation: one turn
/// per cycle, shaped by `easing`, or none at all under reduced motion.
pub(crate) fn spin(motion: Motion, easing: Easing) -> f32 {
    if motion.reduced() {
        0.0
    } else {
        360.0 * easing.turns(motion.progress())
    }
}

/// A dash of `length` along a circle centered in a `view`-unit box, from
/// 3 o'clock clockwise — what a dashed SVG `<circle>` draws.
pub(crate) fn arc(view: f32, radius: f32, width: f32, length: f32, cap: Cap) -> Sprite {
    Sprite::Arc {
        view: milli(view),
        radius: milli(radius),
        width: milli(width),
        length: milli(length),
        cap,
    }
}
