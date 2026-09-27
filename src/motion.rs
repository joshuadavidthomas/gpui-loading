//! The motion contract: each spinner's default duration, playback, and the
//! clock every animation reads its progress from.

use std::time::Duration;
use std::time::Instant;

use gpui::App;
use gpui::Global;

/// The size a spinner renders at when none is given, in pixels.
pub const DEFAULT_SIZE: f32 = 20.0;

/// Whether a spinner's animation is advancing. Pausing keeps a spinner at
/// its current frame and resuming continues from there.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PlayState {
    #[default]
    Running,
    Paused,
}

macro_rules! spinner_names {
    ($($variant:ident => $key:literal, $ms:literal;)*) => {
        /// Every spinner the library ships, under its `ld-` key.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum SpinnerName {
            $($variant,)*
        }

        impl SpinnerName {
            pub const ALL: &'static [SpinnerName] = &[$(SpinnerName::$variant,)*];

            /// The spinner's `ld-` key, e.g. `"bouncing-dots"`.
            pub const fn key(self) -> &'static str {
                match self {
                    $(SpinnerName::$variant => $key,)*
                }
            }

            /// The component's name, e.g. `"BouncingDots"`.
            pub const fn component(self) -> &'static str {
                match self {
                    $(SpinnerName::$variant => stringify!($variant),)*
                }
            }

            /// The length of one cycle when no `duration` is given.
            pub const fn default_duration(self) -> Duration {
                match self {
                    $(SpinnerName::$variant => Duration::from_millis($ms),)*
                }
            }

            pub fn from_key(key: &str) -> Option<SpinnerName> {
                match key {
                    $($key => Some(SpinnerName::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

spinner_names! {
    Arc => "arc", 800;
    Atom => "atom", 1000;
    Blocks => "blocks", 1300;
    BouncingDots => "bouncing-dots", 500;
    Cascade => "cascade", 1500;
    CircularDots => "circular-dots", 800;
    Classic => "classic", 1200;
    ClassicV2 => "classic-v2", 800;
    Clock => "clock", 1200;
    Comet => "comet", 700;
    Compass => "compass", 500;
    Dual => "dual", 1000;
    Eclipse => "eclipse", 1200;
    Flip => "flip", 1200;
    Gather => "gather", 1600;
    Leap => "leap", 1800;
    LinearDots => "linear-dots", 900;
    Loading => "loading", 1000;
    Morph => "morph", 1200;
    Orbit => "orbit", 750;
    Pulse => "pulse", 1200;
    Radar => "radar", 1500;
    Ring => "ring", 800;
    Ripple => "ripple", 1200;
    Slide => "slide", 2400;
    Snake => "snake", 1400;
    Swirl => "swirl", 1200;
    Trace => "trace", 1200;
    Wave => "wave", 900;
}

/// App-wide reduced-motion preference, named after the values of CSS's
/// `prefers-reduced-motion`. Under [`ReducedMotion::Reduce`] every spinner
/// shows a still frame instead of animating, as the web version does. GPUI
/// does not read the platform setting, so the app decides.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReducedMotion {
    #[default]
    NoPreference,
    Reduce,
}

impl Global for ReducedMotion {}

pub fn set_reduced_motion(cx: &mut App, preference: ReducedMotion) {
    cx.set_global(preference);
}

pub fn reduced_motion(cx: &App) -> ReducedMotion {
    cx.try_global::<ReducedMotion>()
        .copied()
        .unwrap_or_default()
}

/// Per-spinner elapsed time that stops while paused.
#[derive(Default)]
pub(crate) struct Clock {
    banked: Duration,
    running_since: Option<Instant>,
}

impl Clock {
    pub fn tick(&mut self, play_state: PlayState) -> Duration {
        let now = Instant::now();
        match (play_state, self.running_since) {
            (PlayState::Running, None) => self.running_since = Some(now),
            (PlayState::Paused, Some(since)) => {
                self.banked += now - since;
                self.running_since = None;
            }
            _ => {}
        }
        self.banked
            + self
                .running_since
                .map_or(Duration::ZERO, |since| now - since)
    }
}

/// Where a spinner is in its animation, handed to its painter each frame.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Motion {
    /// Elapsed time in cycles.
    cycles: f64,
    preference: ReducedMotion,
}

impl Motion {
    pub fn new(elapsed: Duration, duration: Duration, preference: ReducedMotion) -> Motion {
        let cycles = elapsed.as_secs_f64() / duration.as_secs_f64().max(1e-3);
        Motion { cycles, preference }
    }

    /// Whether to paint the still frame used for reduced motion.
    pub fn reduced(&self) -> bool {
        self.preference == ReducedMotion::Reduce
    }

    /// Animation time, in cycles, of element `step` of `count` staggered
    /// ones. Matches `animation-delay: duration * (step - count) / count`:
    /// a negative delay, so every element is mid-animation from the start.
    #[expect(
        clippy::cast_precision_loss,
        reason = "a spinner staggers only a handful of elements"
    )]
    fn time(&self, step: usize, count: usize) -> f64 {
        self.cycles + (count - step) as f64 / count as f64
    }

    /// Progress through the current cycle, `0.0..1.0`.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "a fraction of a cycle needs only f32 precision"
    )]
    pub fn progress(&self) -> f32 {
        self.cycles.fract() as f32
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "a fraction of a cycle needs only f32 precision"
    )]
    pub fn staggered(&self, step: usize, count: usize) -> f32 {
        self.time(step, count).fract() as f32
    }

    /// Progress with `animation-direction: alternate`.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "a fraction of a cycle needs only f32 precision"
    )]
    pub fn alternating(&self, step: usize, count: usize) -> f32 {
        let time = self.time(step, count);
        let progress = time.fract() as f32;
        if time.rem_euclid(2.0) >= 1.0 {
            1.0 - progress
        } else {
            progress
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_round_trip() {
        for &name in SpinnerName::ALL {
            assert_eq!(SpinnerName::from_key(name.key()), Some(name));
        }
        assert_eq!(SpinnerName::ALL.len(), 29);
    }

    #[test]
    fn stagger_matches_negative_delay() {
        let motion = Motion::new(
            Duration::ZERO,
            Duration::from_secs(1),
            ReducedMotion::NoPreference,
        );
        assert!(motion.staggered(0, 4).abs() < 1e-6);
        assert!((motion.staggered(1, 4) - 0.75).abs() < 1e-6);
        assert!((motion.staggered(3, 4) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn paused_clock_holds_its_time() {
        let mut clock = Clock::default();
        clock.tick(PlayState::Running);
        std::thread::sleep(Duration::from_millis(5));
        let paused = clock.tick(PlayState::Paused);
        std::thread::sleep(Duration::from_millis(5));
        assert_eq!(clock.tick(PlayState::Paused), paused);
        assert!(clock.tick(PlayState::Running) >= paused);
    }
}
