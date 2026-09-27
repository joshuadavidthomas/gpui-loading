//! Beautiful loading indicators for [GPUI](https://gpui.rs), ported from
//! [loading.dev](https://loading.dev).
//!
//! ```ignore
//! use gpui_loading::Arc;
//!
//! Arc::new("saving").size(px(16.))
//! ```
//!
//! Every spinner takes `size`, `color`, `duration` and `play_state`, and
//! implements [`gpui::Styled`] for margins, opacity and the like. Some also
//! take `easing`, `cap` or an option of their own. Each needs an id, unique
//! among its siblings, to keep its animation clock between frames.

mod cap;
mod easing;
mod frame;
mod geom;
mod motion;
mod spinners;
mod sprite;

pub use cap::Cap;
pub use cap::DEFAULT_CAP;
pub use easing::DEFAULT_EASING;
pub use easing::Easing;
pub use frame::SpinnerProps;
use frame::shared_props;
use gpui::AnyElement;
use gpui::App;
use gpui::ElementId;
use gpui::IntoElement;
use gpui::RenderOnce;
use gpui::Window;
pub use motion::DEFAULT_SIZE;
pub use motion::PlayState;
pub use motion::ReducedMotion;
pub use motion::SpinnerName;
pub use motion::reduced_motion;
pub use motion::set_reduced_motion;
pub use spinners::*;
pub use sprite::SpinnerAssets;

/// Any spinner, chosen by name, with its default options — the registry
/// the web version exports as `SPINNERS`.
#[derive(IntoElement)]
pub struct AnySpinner {
    name: SpinnerName,
    props: SpinnerProps,
}

impl AnySpinner {
    pub fn named(name: SpinnerName, id: impl Into<ElementId>) -> Self {
        AnySpinner {
            name,
            props: SpinnerProps::new(id),
        }
    }
}

shared_props!(AnySpinner, props);

impl RenderOnce for AnySpinner {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let props = self.props;
        let element: AnyElement = match self.name {
            SpinnerName::Arc => Arc::from_props(props).into_any_element(),
            SpinnerName::Atom => Atom::from_props(props).into_any_element(),
            SpinnerName::Blocks => Blocks::from_props(props).into_any_element(),
            SpinnerName::BouncingDots => BouncingDots::from_props(props).into_any_element(),
            SpinnerName::Cascade => Cascade::from_props(props).into_any_element(),
            SpinnerName::CircularDots => CircularDots::from_props(props).into_any_element(),
            SpinnerName::Classic => Classic::from_props(props).into_any_element(),
            SpinnerName::ClassicV2 => ClassicV2::from_props(props).into_any_element(),
            SpinnerName::Clock => Clock::from_props(props).into_any_element(),
            SpinnerName::Comet => Comet::from_props(props).into_any_element(),
            SpinnerName::Compass => Compass::from_props(props).into_any_element(),
            SpinnerName::Dual => Dual::from_props(props).into_any_element(),
            SpinnerName::Eclipse => Eclipse::from_props(props).into_any_element(),
            SpinnerName::Flip => Flip::from_props(props).into_any_element(),
            SpinnerName::Gather => Gather::from_props(props).into_any_element(),
            SpinnerName::Leap => Leap::from_props(props).into_any_element(),
            SpinnerName::LinearDots => LinearDots::from_props(props).into_any_element(),
            SpinnerName::Loading => Loading::from_props(props).into_any_element(),
            SpinnerName::Morph => Morph::from_props(props).into_any_element(),
            SpinnerName::Orbit => Orbit::from_props(props).into_any_element(),
            SpinnerName::Pulse => Pulse::from_props(props).into_any_element(),
            SpinnerName::Radar => Radar::from_props(props).into_any_element(),
            SpinnerName::Ring => Ring::from_props(props).into_any_element(),
            SpinnerName::Ripple => Ripple::from_props(props).into_any_element(),
            SpinnerName::Slide => Slide::from_props(props).into_any_element(),
            SpinnerName::Snake => Snake::from_props(props).into_any_element(),
            SpinnerName::Swirl => Swirl::from_props(props).into_any_element(),
            SpinnerName::Trace => Trace::from_props(props).into_any_element(),
            SpinnerName::Wave => Wave::from_props(props).into_any_element(),
        };
        element
    }
}
