//! A spinner's page: a live preview beside the controls that tune it.

use std::time::Duration;

use gpui_kit::App;
use gpui_kit::ClickEvent;
use gpui_kit::Context;
use gpui_kit::FontWeight;
use gpui_kit::component::Colorize;
use gpui_kit::component::Icon;
use gpui_kit::component::IconName;
use gpui_kit::component::Sizable;
use gpui_kit::component::color_picker::ColorPicker;
use gpui_kit::component::color_picker::ColorPickerState;
use gpui_kit::component::h_flex;
use gpui_kit::component::v_flex;
use gpui_kit::div;
use gpui_kit::prelude::*;
use gpui_kit::px;
use gpui_kit::rgb;
use gpui_loading::AnySpinner;
use gpui_loading::PlayState;
use gpui_loading::SpinnerName;

use crate::code::code_block;
use crate::controls::icon_button;
use crate::controls::slider;
use crate::gallery::Gallery;
use crate::spinners::description;
use crate::spinners::title;
use crate::theme::ORANGE;
use crate::theme::palette;

pub(crate) const SIZES: [(&str, f32); 3] = [("Small", 24.0), ("Medium", 48.0), ("Large", 96.0)];

/// The colours the picker offers up front, after the inherited text colour.
pub(crate) const FEATURED_COLORS: [u32; 5] =
    [ORANGE, 0x003b_82f6, 0x0022_c55e, 0x00a8_55f7, 0x00ec_4899];

/// The speed slider's range, in milliseconds per cycle. The slider runs
/// the other way, as on the site: right is faster.
pub(crate) const MIN_MS: f64 = 200.0;
pub(crate) const MAX_MS: f64 = 2000.0;

/// `duration` in whole milliseconds. The speed slider stores it as a float,
/// so compare and print it rounded.
pub(crate) fn millis(duration: Duration) -> f64 {
    (duration.as_secs_f64() * 1000.0).round()
}

impl Gallery {
    /// Laid out as on the site: the preview on the page's own colour, framed
    /// and bordered, with the code below it on the frame's tint.
    pub(crate) fn spinner_page(
        &self,
        name: SpinnerName,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let palette = palette(cx);
        let snippet = self.snippet(name, cx);

        let preview = v_flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .px_4()
            .pt(px(52.))
            .pb_2()
            .child(
                div()
                    .flex()
                    .flex_1()
                    .w_full()
                    .items_center()
                    .justify_center()
                    .child(self.configured(name, cx)),
            )
            .child(
                icon_button("pause", &palette)
                    .child(Icon::new(if self.paused {
                        IconName::Play
                    } else {
                        IconName::Pause
                    }))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.paused = !this.paused;
                        cx.notify();
                    })),
            );

        // Tucked against the preview's edge, as on the site.
        let toggle = div()
            .id("toggle-panel")
            .flex()
            .flex_none()
            .items_center()
            .justify_end()
            .h_full()
            .w_9()
            .ml(px(-20.))
            .mr(px(-4.))
            .pr_1p5()
            .text_color(palette.muted)
            .cursor_pointer()
            .hover(|style| style.text_color(palette.text))
            .child(Icon::new(if self.panel_open {
                IconName::ChevronRight
            } else {
                IconName::ChevronLeft
            }))
            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.panel_open = !this.panel_open;
                cx.notify();
            }));

        v_flex()
            .gap_8()
            .child(
                v_flex()
                    .gap_5()
                    .px_4()
                    .child(
                        div()
                            .text_size(px(36.))
                            .line_height(px(40.))
                            .font_weight(FontWeight::MEDIUM)
                            .child(div().text_color(palette.muted).child("Component/"))
                            .child(title(name)),
                    )
                    .child(div().text_color(palette.muted).child(description(name))),
            )
            .child(
                v_flex()
                    .rounded_2xl()
                    .bg(palette.subtle)
                    .child(
                        h_flex()
                            .items_stretch()
                            .h(px(400.))
                            .p_1()
                            .rounded_2xl()
                            .border_1()
                            .border_color(palette.border)
                            .bg(palette.surface)
                            .child(preview)
                            .child(toggle)
                            .when(self.panel_open, |this| this.child(self.panel(name, cx))),
                    )
                    .child(code_block(&snippet, &palette)),
            )
    }

    /// The spinner as the controls have it.
    pub(crate) fn configured(&self, name: SpinnerName, cx: &App) -> AnySpinner {
        let mut spinner = AnySpinner::named(name, "preview")
            .size(px(SIZES[self.size].1))
            .duration(self.duration)
            .opacity(self.opacity)
            .play_state(if self.paused {
                PlayState::Paused
            } else {
                PlayState::Running
            });
        if let Some(color) = self.color(cx) {
            spinner = spinner.color(color);
        }
        spinner
    }

    pub(crate) fn panel(&self, name: SpinnerName, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = palette(cx);
        let ms = millis(self.duration);
        let hex = self
            .color
            .read(cx)
            .value()
            .map(|color| color.to_hex().to_uppercase())
            .unwrap_or_default();

        let sizes = SIZES.iter().enumerate().map(|(index, (label, _))| {
            let selected = self.size == index;
            div()
                .id(("size", index))
                .flex()
                .flex_1()
                .min_w_0()
                .h_8()
                .items_center()
                .justify_center()
                .rounded_lg()
                .cursor_pointer()
                .when(selected, |this| {
                    this.bg(palette.raised)
                        .border_1()
                        .border_color(palette.border)
                })
                .when(!selected, |this| {
                    this.text_color(palette.muted)
                        .hover(|style| style.text_color(palette.text))
                })
                .child(*label)
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.size = index;
                    cx.notify();
                }))
        });

        let color = h_flex()
            .h_8()
            .gap_2()
            .pl_2()
            .pr_2()
            .rounded_lg()
            .bg(palette.raised)
            .child(
                h_flex()
                    .id("color")
                    .group("color")
                    .flex_1()
                    .h_full()
                    .justify_between()
                    .cursor_pointer()
                    .child(
                        div()
                            .text_color(palette.muted)
                            .group_hover("color", |style| style.text_color(palette.text))
                            .child("Color"),
                    )
                    .child(
                        div()
                            .font_family(palette.mono.clone())
                            .text_size(px(12.))
                            .text_color(palette.muted)
                            .group_hover("color", |style| style.text_color(palette.text))
                            .child(hex),
                    )
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.color.update(cx, ColorPickerState::toggle_open);
                    })),
            )
            .child(
                ColorPicker::new(&self.color).xsmall().featured_colors(
                    FEATURED_COLORS
                        .iter()
                        .map(|&color| rgb(color).into())
                        .collect(),
                ),
            );

        v_flex()
            .flex_none()
            .w(px(240.))
            .h_full()
            .gap_2()
            .p_2()
            .rounded_xl()
            .bg(palette.raised)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .child(
                h_flex()
                    .h_8()
                    .rounded_lg()
                    .bg(palette.raised)
                    .children(sizes),
            )
            .child(color)
            .child(slider(
                "speed",
                "Speed",
                format!("{ms:.0}ms"),
                // Right is faster, as on the site.
                (MAX_MS - ms) / (MAX_MS - MIN_MS),
                &palette,
                cx,
                |this, fraction| {
                    let ms = MAX_MS - fraction * (MAX_MS - MIN_MS);
                    this.duration = Duration::from_secs_f64((ms / 10.0).round() / 100.0);
                },
            ))
            .child(slider(
                "opacity",
                "Opacity",
                format!("{:.0}%", self.opacity * 100.0),
                f64::from(self.opacity),
                &palette,
                cx,
                |this, fraction| {
                    // Two decimal places, from a value already in 0..=1.
                    #[allow(clippy::cast_possible_truncation)]
                    let opacity = ((fraction * 100.0).round() / 100.0) as f32;
                    this.opacity = opacity;
                },
            ))
            .child(
                h_flex().mt_auto().justify_center().child(
                    h_flex()
                        .id("reset")
                        .h_8()
                        .gap_1p5()
                        .px_2p5()
                        .rounded_lg()
                        .cursor_pointer()
                        .hover(|style| style.bg(palette.raised))
                        .child(Icon::new(IconName::Undo2).text_color(palette.muted))
                        .child("Reset")
                        .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                            this.reset(name, window, cx);
                            cx.notify();
                        })),
                ),
            )
    }
}
