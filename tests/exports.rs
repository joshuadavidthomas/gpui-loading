//! Every public export, used the way an app would use it — the port of the
//! web version's export tests.

use std::time::Duration;

use gpui::AnyElement;
use gpui::IntoElement;
use gpui::px;
use gpui::rgb;
use gpui_loading::AnySpinner;
use gpui_loading::Arc;
use gpui_loading::Atom;
use gpui_loading::Blocks;
use gpui_loading::BlocksSweep;
use gpui_loading::BouncingDots;
use gpui_loading::Cap;
use gpui_loading::Cascade;
use gpui_loading::CircularDots;
use gpui_loading::Classic;
use gpui_loading::ClassicV2;
use gpui_loading::Clock;
use gpui_loading::Comet;
use gpui_loading::Compass;
use gpui_loading::DEFAULT_BLOCKS_SWEEP;
use gpui_loading::DEFAULT_CAP;
use gpui_loading::DEFAULT_EASING;
use gpui_loading::DEFAULT_RIPPLE_DIRECTION;
use gpui_loading::DEFAULT_SIZE;
use gpui_loading::DEFAULT_WAVE_ORIGIN;
use gpui_loading::Dual;
use gpui_loading::Easing;
use gpui_loading::Eclipse;
use gpui_loading::Flip;
use gpui_loading::Gather;
use gpui_loading::Leap;
use gpui_loading::LinearDots;
use gpui_loading::Loading;
use gpui_loading::Morph;
use gpui_loading::Orbit;
use gpui_loading::PlayState;
use gpui_loading::Pulse;
use gpui_loading::Radar;
use gpui_loading::Ring;
use gpui_loading::Ripple;
use gpui_loading::RippleDirection;
use gpui_loading::Slide;
use gpui_loading::Snake;
use gpui_loading::SpinnerAssets;
use gpui_loading::SpinnerName;
use gpui_loading::Swirl;
use gpui_loading::Trace;
use gpui_loading::Wave;
use gpui_loading::WaveOrigin;

/// One of each spinner, in `SpinnerName` order, with every option it takes.
#[expect(
    clippy::too_many_lines,
    reason = "one entry per spinner reads best as a single table"
)]
fn every_spinner() -> Vec<(SpinnerName, AnyElement)> {
    vec![
        (
            SpinnerName::Arc,
            Arc::new("arc")
                .easing(Easing::Stacked)
                .cap(Cap::Flat)
                .into_any_element(),
        ),
        (
            SpinnerName::Atom,
            Atom::new("atom")
                .easing(Easing::EaseInOut)
                .into_any_element(),
        ),
        (
            SpinnerName::Blocks,
            Blocks::new("blocks")
                .sweep(BlocksSweep::Rows)
                .into_any_element(),
        ),
        (
            SpinnerName::BouncingDots,
            BouncingDots::new("bouncing-dots").into_any_element(),
        ),
        (
            SpinnerName::Cascade,
            Cascade::new("cascade").cap(Cap::Flat).into_any_element(),
        ),
        (
            SpinnerName::CircularDots,
            CircularDots::new("circular-dots").into_any_element(),
        ),
        (
            SpinnerName::Classic,
            Classic::new("classic").into_any_element(),
        ),
        (
            SpinnerName::ClassicV2,
            ClassicV2::new("classic-v2").into_any_element(),
        ),
        (
            SpinnerName::Clock,
            Clock::new("clock")
                .easing(Easing::Linear)
                .into_any_element(),
        ),
        (
            SpinnerName::Comet,
            Comet::new("comet")
                .easing(Easing::EaseInOut)
                .into_any_element(),
        ),
        (
            SpinnerName::Compass,
            Compass::new("compass").into_any_element(),
        ),
        (
            SpinnerName::Dual,
            Dual::new("dual")
                .easing(Easing::Stacked)
                .cap(Cap::Round)
                .into_any_element(),
        ),
        (
            SpinnerName::Eclipse,
            Eclipse::new("eclipse").into_any_element(),
        ),
        (SpinnerName::Flip, Flip::new("flip").into_any_element()),
        (
            SpinnerName::Gather,
            Gather::new("gather").into_any_element(),
        ),
        (SpinnerName::Leap, Leap::new("leap").into_any_element()),
        (
            SpinnerName::LinearDots,
            LinearDots::new("linear-dots").into_any_element(),
        ),
        (
            SpinnerName::Loading,
            Loading::new("loading").into_any_element(),
        ),
        (SpinnerName::Morph, Morph::new("morph").into_any_element()),
        (
            SpinnerName::Orbit,
            Orbit::new("orbit")
                .easing(Easing::EaseInOut)
                .into_any_element(),
        ),
        (SpinnerName::Pulse, Pulse::new("pulse").into_any_element()),
        (
            SpinnerName::Radar,
            Radar::new("radar")
                .easing(Easing::EaseInOut)
                .into_any_element(),
        ),
        (
            SpinnerName::Ring,
            Ring::new("ring")
                .easing(Easing::EaseInOut)
                .cap(Cap::Flat)
                .into_any_element(),
        ),
        (
            SpinnerName::Ripple,
            Ripple::new("ripple")
                .direction(RippleDirection::In)
                .into_any_element(),
        ),
        (SpinnerName::Slide, Slide::new("slide").into_any_element()),
        (
            SpinnerName::Snake,
            Snake::new("snake")
                .easing(Easing::EaseInOut)
                .cap(Cap::Flat)
                .into_any_element(),
        ),
        (SpinnerName::Swirl, Swirl::new("swirl").into_any_element()),
        (
            SpinnerName::Trace,
            Trace::new("trace")
                .easing(Easing::Stacked)
                .cap(Cap::Flat)
                .into_any_element(),
        ),
        (
            SpinnerName::Wave,
            Wave::new("wave")
                .origin(WaveOrigin::Bottom)
                .into_any_element(),
        ),
    ]
}

#[test]
fn every_spinner_is_exported_under_its_name() {
    let names: Vec<SpinnerName> = every_spinner().into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, SpinnerName::ALL);
}

#[test]
fn any_spinner_takes_the_shared_props() {
    let spinners: Vec<AnyElement> = SpinnerName::ALL
        .iter()
        .map(|&name| {
            AnySpinner::named(name, name.key())
                .size(px(DEFAULT_SIZE * 2.0))
                .color(rgb(0x003b_82f6))
                .duration(Duration::from_millis(500))
                .play_state(PlayState::Paused)
                .into_any_element()
        })
        .collect();
    assert_eq!(spinners.len(), SpinnerName::ALL.len());
}

#[test]
fn defaults_are_among_the_options() {
    assert!(Cap::ALL.contains(&DEFAULT_CAP));
    assert!(Easing::ALL.contains(&DEFAULT_EASING));
    assert!(BlocksSweep::ALL.contains(&DEFAULT_BLOCKS_SWEEP));
    assert!(RippleDirection::ALL.contains(&DEFAULT_RIPPLE_DIRECTION));
    assert!(WaveOrigin::ALL.contains(&DEFAULT_WAVE_ORIGIN));
}

#[test]
fn assets_can_wrap_the_apps_own() {
    let wrapped = SpinnerAssets::wrap(SpinnerAssets::new());
    assert!(matches!(
        gpui::AssetSource::load(&wrapped, "icons/app.svg"),
        Ok(None)
    ));
}
