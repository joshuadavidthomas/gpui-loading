use super::prelude::*;

/// Where the bars of [`Wave`] grow from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WaveOrigin {
    Bottom,
    #[default]
    Center,
}

pub const DEFAULT_WAVE_ORIGIN: WaveOrigin = WaveOrigin::Center;

impl WaveOrigin {
    pub const ALL: [WaveOrigin; 2] = [WaveOrigin::Bottom, WaveOrigin::Center];
}

const BARS: u16 = 5;

const BAR: f32 = 0.12;

/// Five bars rising and falling in a wave.
#[derive(IntoElement)]
pub struct Wave {
    props: SpinnerProps,
    origin: WaveOrigin,
}

spinner_props!(Wave, origin);

impl Wave {
    #[must_use]
    pub fn origin(mut self, origin: WaveOrigin) -> Self {
        self.origin = origin;
        self
    }
}

impl RenderOnce for Wave {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Wave { props, origin } = self;
        frame(
            props,
            SpinnerName::Wave,
            (1.0, 1.0),
            window,
            cx,
            move |p, m| {
                for index in 0..BARS {
                    let height = if m.reduced() {
                        0.4 + f32::from(index) * 0.15
                    } else {
                        keyframes(
                            m.staggered(index.into(), BARS.into()),
                            &[(0.0, 0.3), (0.5, 1.0), (1.0, 0.3)],
                            Timing::EASE_IN_OUT,
                        )
                    };
                    let x = f32::from(index) * (1.0 - BAR) / f32::from(BARS - 1);
                    let y = match origin {
                        WaveOrigin::Bottom => 1.0 - height,
                        WaveOrigin::Center => (1.0 - height) / 2.0,
                    };
                    p.rect(x, y, BAR, height, BAR / 2.0, 1.0);
                }
            },
        )
    }
}
