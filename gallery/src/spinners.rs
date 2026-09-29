//! What the site calls each spinner and how it describes it.

use gpui_loading::SpinnerName;

/// The site's name for a spinner: its key in sentence case, e.g.
/// `"Bouncing dots"`.
pub(crate) fn title(name: SpinnerName) -> String {
    let key = name.key().replace('-', " ");
    let mut chars = key.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

pub(crate) fn description(name: SpinnerName) -> &'static str {
    match name {
        SpinnerName::Arc => "A single open stroke rotating in a circle.",
        SpinnerName::Atom => "Three rings tumbling inside a circle.",
        SpinnerName::Blocks => "Nine blocks shrinking and growing in a sweep across a grid.",
        SpinnerName::BouncingDots => "Three staggered dots bouncing up and down.",
        SpinnerName::Cascade => {
            "Three nested arcs fanning into a spiral and snapping back in line."
        }
        SpinnerName::CircularDots => "Eight dots in a ring, the brightest hopping around.",
        SpinnerName::Classic => "Twelve fading bars arranged in a radial pattern.",
        SpinnerName::ClassicV2 => "Two lit ticks stepping around a ring of eight.",
        SpinnerName::Clock => "A clock hand sweeping around a faint face.",
        SpinnerName::Comet => "A full ring fading into its tail.",
        SpinnerName::Compass => "Four ticks snapping a quarter turn at a time.",
        SpinnerName::Dual => "Two arcs turning in opposite directions.",
        SpinnerName::Eclipse => "Two dots trading places, one passing behind the other.",
        SpinnerName::Flip => "A square flipping over on one axis, then the other.",
        SpinnerName::Gather => "Four blocks pulling together, turning, and pushing apart.",
        SpinnerName::Leap => "Three dots in a row, the last one leaping to the front.",
        SpinnerName::LinearDots => "Three dots lighting up in turn from left to right.",
        SpinnerName::Loading => "The loading.dev mark, its brightest block circling the ring.",
        SpinnerName::Morph => "A square rounding into a circle and back as it turns.",
        SpinnerName::Orbit => "A fading half-arc rotating around a dot.",
        SpinnerName::Pulse => "A ring rippling outward from a dot.",
        SpinnerName::Radar => "A beam sweeping a scope, fading toward its edge.",
        SpinnerName::Ring => "An arc rotating in a faint circle.",
        SpinnerName::Ripple => "Three rings spreading out from the center.",
        SpinnerName::Slide => "Three dots sliding into the empty corner of a square.",
        SpinnerName::Snake => "An arc stretching and shrinking as it circles.",
        SpinnerName::Swirl => "A bright cell chasing its trail around a square.",
        SpinnerName::Trace => "A dash tracing the outline of a rounded square.",
        SpinnerName::Wave => "Five bars rising and falling in a wave.",
    }
}
