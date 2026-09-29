//! Upstream's per-spinner customization controls and speed ranges.

use gpui_loading::AnySpinner;
use gpui_loading::BlocksSweep;
use gpui_loading::Cap;
use gpui_loading::Easing;
use gpui_loading::RippleDirection;
use gpui_loading::SpinnerName;
use gpui_loading::WaveOrigin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OptionValue {
    Easing(Easing),
    Cap(Cap),
    Sweep(BlocksSweep),
    Direction(RippleDirection),
    Origin(WaveOrigin),
}

impl OptionValue {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Easing(Easing::Linear) => "Linear",
            Self::Easing(Easing::EaseInOut) => "Eased",
            Self::Easing(Easing::Stacked) => "Stacked",
            Self::Cap(Cap::Round) => "Round",
            Self::Cap(Cap::Flat) => "Flat",
            Self::Sweep(BlocksSweep::Diagonal) => "Diagonal",
            Self::Sweep(BlocksSweep::Rows) => "Rows",
            Self::Sweep(BlocksSweep::Columns) => "Columns",
            Self::Direction(RippleDirection::Out) => "Out",
            Self::Direction(RippleDirection::In) => "In",
            Self::Origin(WaveOrigin::Center) => "Center",
            Self::Origin(WaveOrigin::Bottom) => "Bottom",
        }
    }

    pub(crate) fn rust_value(self) -> (&'static str, &'static str) {
        match self {
            Self::Easing(Easing::Linear) => ("Easing", "Linear"),
            Self::Easing(Easing::EaseInOut) => ("Easing", "EaseInOut"),
            Self::Easing(Easing::Stacked) => ("Easing", "Stacked"),
            Self::Cap(Cap::Round) => ("Cap", "Round"),
            Self::Cap(Cap::Flat) => ("Cap", "Flat"),
            Self::Sweep(BlocksSweep::Diagonal) => ("BlocksSweep", "Diagonal"),
            Self::Sweep(BlocksSweep::Rows) => ("BlocksSweep", "Rows"),
            Self::Sweep(BlocksSweep::Columns) => ("BlocksSweep", "Columns"),
            Self::Direction(RippleDirection::Out) => ("RippleDirection", "Out"),
            Self::Direction(RippleDirection::In) => ("RippleDirection", "In"),
            Self::Origin(WaveOrigin::Center) => ("WaveOrigin", "Center"),
            Self::Origin(WaveOrigin::Bottom) => ("WaveOrigin", "Bottom"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Control {
    Easing,
    Cap,
    Sweep,
    Direction,
    Origin,
}

impl Control {
    pub(crate) fn prop(self) -> &'static str {
        match self {
            Self::Easing => "easing",
            Self::Cap => "cap",
            Self::Sweep => "sweep",
            Self::Direction => "direction",
            Self::Origin => "origin",
        }
    }

    pub(crate) fn values(self) -> &'static [OptionValue] {
        match self {
            Self::Easing => &[
                OptionValue::Easing(Easing::Linear),
                OptionValue::Easing(Easing::EaseInOut),
                OptionValue::Easing(Easing::Stacked),
            ],
            Self::Cap => &[OptionValue::Cap(Cap::Round), OptionValue::Cap(Cap::Flat)],
            Self::Sweep => &[
                OptionValue::Sweep(BlocksSweep::Diagonal),
                OptionValue::Sweep(BlocksSweep::Rows),
                OptionValue::Sweep(BlocksSweep::Columns),
            ],
            Self::Direction => &[
                OptionValue::Direction(RippleDirection::Out),
                OptionValue::Direction(RippleDirection::In),
            ],
            Self::Origin => &[
                OptionValue::Origin(WaveOrigin::Center),
                OptionValue::Origin(WaveOrigin::Bottom),
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Options {
    easing: Easing,
    cap: Cap,
    sweep: BlocksSweep,
    direction: RippleDirection,
    origin: WaveOrigin,
}

impl Options {
    pub(crate) fn value(self, control: Control) -> OptionValue {
        match control {
            Control::Easing => OptionValue::Easing(self.easing),
            Control::Cap => OptionValue::Cap(self.cap),
            Control::Sweep => OptionValue::Sweep(self.sweep),
            Control::Direction => OptionValue::Direction(self.direction),
            Control::Origin => OptionValue::Origin(self.origin),
        }
    }

    pub(crate) fn set(&mut self, value: OptionValue) {
        match value {
            OptionValue::Easing(value) => self.easing = value,
            OptionValue::Cap(value) => self.cap = value,
            OptionValue::Sweep(value) => self.sweep = value,
            OptionValue::Direction(value) => self.direction = value,
            OptionValue::Origin(value) => self.origin = value,
        }
    }

    pub(crate) fn apply(self, spinner: AnySpinner) -> AnySpinner {
        spinner
            .easing(self.easing)
            .cap(self.cap)
            .sweep(self.sweep)
            .direction(self.direction)
            .origin(self.origin)
    }

    /// Only supported, non-default settings belong in the copied example.
    pub(crate) fn overrides(self, name: SpinnerName) -> Vec<(Control, OptionValue)> {
        controls(name)
            .iter()
            .copied()
            .filter_map(|control| {
                let value = self.value(control);
                (value != Self::default().value(control)).then_some((control, value))
            })
            .collect()
    }
}

pub(crate) fn controls(name: SpinnerName) -> &'static [Control] {
    match name {
        SpinnerName::Arc
        | SpinnerName::Dual
        | SpinnerName::Ring
        | SpinnerName::Snake
        | SpinnerName::Trace => &[Control::Easing, Control::Cap],
        SpinnerName::Atom
        | SpinnerName::Clock
        | SpinnerName::Comet
        | SpinnerName::Orbit
        | SpinnerName::Radar => &[Control::Easing],
        SpinnerName::Cascade => &[Control::Cap],
        SpinnerName::Blocks => &[Control::Sweep],
        SpinnerName::Ripple => &[Control::Direction],
        SpinnerName::Wave => &[Control::Origin],
        SpinnerName::BouncingDots
        | SpinnerName::CircularDots
        | SpinnerName::Classic
        | SpinnerName::ClassicV2
        | SpinnerName::Compass
        | SpinnerName::Eclipse
        | SpinnerName::Flip
        | SpinnerName::Gather
        | SpinnerName::Leap
        | SpinnerName::LinearDots
        | SpinnerName::Loading
        | SpinnerName::Morph
        | SpinnerName::Pulse
        | SpinnerName::Slide
        | SpinnerName::Swirl => &[],
    }
}

/// Milliseconds per cycle, as in the upstream web catalog. Compass and
/// Radar are unlisted upstream, so retain their existing gallery range.
pub(crate) fn speed_range(name: SpinnerName) -> (f64, f64) {
    match name {
        SpinnerName::Arc
        | SpinnerName::CircularDots
        | SpinnerName::Comet
        | SpinnerName::Orbit
        | SpinnerName::Ring
        | SpinnerName::Compass
        | SpinnerName::Radar => (200.0, 2000.0),
        SpinnerName::Atom
        | SpinnerName::Eclipse
        | SpinnerName::Flip
        | SpinnerName::LinearDots
        | SpinnerName::Loading
        | SpinnerName::Morph
        | SpinnerName::Trace => (300.0, 2400.0),
        SpinnerName::Blocks => (400.0, 2600.0),
        SpinnerName::BouncingDots => (150.0, 1200.0),
        SpinnerName::Cascade => (400.0, 3000.0),
        SpinnerName::Classic | SpinnerName::Pulse | SpinnerName::Ripple | SpinnerName::Swirl => {
            (400.0, 2400.0)
        }
        SpinnerName::ClassicV2 => (400.0, 2000.0),
        SpinnerName::Clock => (300.0, 3000.0),
        SpinnerName::Dual => (250.0, 2000.0),
        SpinnerName::Gather => (400.0, 3200.0),
        SpinnerName::Leap => (500.0, 3600.0),
        SpinnerName::Slide => (600.0, 4800.0),
        SpinnerName::Snake => (400.0, 2800.0),
        SpinnerName::Wave => (300.0, 2000.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_choice_is_selectable_and_resettable() {
        for &name in SpinnerName::ALL {
            let mut options = Options::default();
            assert!(options.overrides(name).is_empty());
            for &control in controls(name) {
                for &value in control.values() {
                    options.set(value);
                    assert_eq!(options.value(control), value);
                    let overrides = options.overrides(name);
                    if value == Options::default().value(control) {
                        assert!(!overrides.iter().any(|&(key, _)| key == control));
                    } else {
                        assert!(overrides.contains(&(control, value)));
                    }
                }
            }
            options = Options::default();
            assert!(options.overrides(name).is_empty());
        }
    }

    #[test]
    fn options_do_not_leak_to_unsupported_spinners() {
        let mut options = Options::default();
        options.set(OptionValue::Easing(Easing::Stacked));
        options.set(OptionValue::Cap(Cap::Flat));
        options.set(OptionValue::Sweep(BlocksSweep::Columns));
        assert_eq!(
            options.overrides(SpinnerName::Arc),
            vec![
                (Control::Easing, OptionValue::Easing(Easing::Stacked)),
                (Control::Cap, OptionValue::Cap(Cap::Flat)),
            ]
        );
        assert_eq!(
            options.overrides(SpinnerName::Blocks),
            vec![(Control::Sweep, OptionValue::Sweep(BlocksSweep::Columns)),]
        );
        assert!(options.overrides(SpinnerName::BouncingDots).is_empty());
    }

    #[test]
    fn speed_ranges_include_every_default() {
        for &name in SpinnerName::ALL {
            let (min, max) = speed_range(name);
            let duration = name.default_duration().as_secs_f64() * 1000.0;
            assert!(min < max);
            assert!((min..=max).contains(&duration), "{}", name.key());
        }
        assert_eq!(speed_range(SpinnerName::BouncingDots), (150.0, 1200.0));
        assert_eq!(speed_range(SpinnerName::Slide), (600.0, 4800.0));
    }
}
