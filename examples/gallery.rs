//! Every spinner in a grid, with the controls the loading.dev showcase has:
//! size, colour, speed, playback and reduced motion.
//!
//! ```sh
//! cargo run --example gallery
//! ```

use std::time::Duration;

use gpui::App;
use gpui::Application;
use gpui::Bounds;
use gpui::Context;
use gpui::Hsla;
use gpui::SharedString;
use gpui::TitlebarOptions;
use gpui::Window;
use gpui::WindowBounds;
use gpui::WindowOptions;
use gpui::div;
use gpui::prelude::*;
use gpui::px;
use gpui::rgb;
use gpui::size;
use gpui_loading::AnySpinner;
use gpui_loading::PlayState;
use gpui_loading::ReducedMotion;
use gpui_loading::SpinnerAssets;
use gpui_loading::SpinnerName;
use gpui_loading::reduced_motion;
use gpui_loading::set_reduced_motion;

const SIZES: [f32; 4] = [20.0, 32.0, 48.0, 64.0];

const SPEEDS: [f32; 3] = [1.0, 0.5, 2.0];

/// Rounded corners and borders together are the priciest thing GPUI's quad
/// shader draws, so the cards and buttons have only one: they stand out from
/// the page by colour instead.
struct Theme {
    background: Hsla,
    card: Hsla,
    text: Hsla,
    muted: Hsla,
}

fn theme(dark: bool) -> Theme {
    if dark {
        Theme {
            background: rgb(0x000a_0a0a).into(),
            card: rgb(0x0016_1616).into(),
            text: rgb(0x00ed_eded).into(),
            muted: rgb(0x008f_8f8f).into(),
        }
    } else {
        Theme {
            background: rgb(0x00f0_f0f0).into(),
            card: rgb(0x00ff_ffff).into(),
            text: rgb(0x0017_1717).into(),
            muted: rgb(0x0073_7373).into(),
        }
    }
}

/// `None` paints with the inherited text colour.
const COLORS: [(&str, Option<u32>); 4] = [
    ("Text", None),
    ("Blue", Some(0x003b_82f6)),
    ("Orange", Some(0x00f9_7316)),
    ("Green", Some(0x0022_c55e)),
];

struct Gallery {
    size: usize,
    color: usize,
    speed: usize,
    paused: bool,
    dark: bool,
}

impl Gallery {
    fn button(
        &self,
        id: &'static str,
        label: impl Into<SharedString>,
        cx: &mut Context<Self>,
        on_click: impl Fn(&mut Self, &mut App) + 'static,
    ) -> impl IntoElement {
        let theme = theme(self.dark);
        div()
            .id(id)
            .px_3()
            .py_1()
            .rounded_md()
            .bg(theme.card)
            .text_sm()
            .cursor_pointer()
            .child(label.into())
            .on_click(cx.listener(move |this, _, _, cx| {
                on_click(this, cx);
                cx.notify();
            }))
    }

    fn toolbar(
        &self,
        speed: f32,
        color_name: &str,
        motion: ReducedMotion,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
            .gap_2()
            .items_center()
            .child(self.button(
                "play",
                if self.paused { "Play" } else { "Pause" },
                cx,
                |this, _| this.paused = !this.paused,
            ))
            .child(self.button(
                "size",
                format!("Size {}px", SIZES[self.size]),
                cx,
                |this, _| this.size = (this.size + 1) % SIZES.len(),
            ))
            .child(
                self.button("speed", format!("Speed {speed}×"), cx, |this, _| {
                    this.speed = (this.speed + 1) % SPEEDS.len();
                }),
            )
            .child(
                self.button("color", format!("Colour: {color_name}"), cx, |this, _| {
                    this.color = (this.color + 1) % COLORS.len();
                }),
            )
            .child(self.button(
                "reduced",
                match motion {
                    ReducedMotion::Reduce => "Reduced motion: on",
                    ReducedMotion::NoPreference => "Reduced motion: off",
                },
                cx,
                |_, cx| {
                    let toggled = match reduced_motion(cx) {
                        ReducedMotion::NoPreference => ReducedMotion::Reduce,
                        ReducedMotion::Reduce => ReducedMotion::NoPreference,
                    };
                    set_reduced_motion(cx, toggled);
                },
            ))
            .child(self.button(
                "theme",
                if self.dark { "Light" } else { "Dark" },
                cx,
                |this, _| this.dark = !this.dark,
            ))
    }
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(self.dark);
        let spinner_size = px(SIZES[self.size]);
        let speed = SPEEDS[self.speed];
        let (color_name, color) = COLORS[self.color];
        let toolbar = self.toolbar(speed, color_name, reduced_motion(cx), cx);

        let cards = SpinnerName::ALL.iter().map(|&name| {
            let mut spinner = AnySpinner::named(name, name.key())
                .size(spinner_size)
                .play_state(if self.paused {
                    PlayState::Paused
                } else {
                    PlayState::Running
                });
            if (speed - 1.0).abs() > f32::EPSILON {
                spinner = spinner.duration(Duration::from_secs_f32(
                    name.default_duration().as_secs_f32() / speed,
                ));
            }
            if let Some(color) = color {
                spinner = spinner.color(rgb(color));
            }
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_between()
                .w(px(132.))
                .h(px(132.))
                .p_3()
                .rounded_lg()
                .bg(theme.card)
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .items_center()
                        .justify_center()
                        .child(spinner),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted)
                        .child(name.component()),
                )
        });

        div()
            .id("gallery")
            .size_full()
            .overflow_y_scroll()
            .bg(theme.background)
            .text_color(theme.text)
            .p_6()
            .flex()
            .flex_col()
            .gap_4()
            .child(div().text_xl().child("loading.dev for GPUI"))
            .child(toolbar)
            .child(div().flex().flex_wrap().gap_3().children(cards))
    }
}

fn main() {
    Application::new()
        .with_assets(SpinnerAssets::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(1080.), px(820.)), cx);
            let window = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Loading".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Gallery {
                        size: 1,
                        color: 0,
                        speed: 0,
                        paused: false,
                        dark: std::env::args().any(|arg| arg == "--dark"),
                    })
                },
            );
            if let Err(error) = window {
                eprintln!("failed to open the gallery window: {error:#}");
                cx.quit();
                return;
            }
            cx.activate(true);
        });
}
