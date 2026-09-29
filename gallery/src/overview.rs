//! The overview: a heading, the install command, and a card per spinner.

use gpui_kit::ClickEvent;
use gpui_kit::Context;
use gpui_kit::FontWeight;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::clipboard::Clipboard;
use gpui_kit::component::h_flex;
use gpui_kit::component::v_flex;
use gpui_kit::div;
use gpui_kit::prelude::*;
use gpui_kit::px;
use gpui_kit::rgb;
use gpui_loading::AnySpinner;
use gpui_loading::SpinnerName;

use crate::gallery::Gallery;
use crate::gallery::Page;
use crate::spinners::title;
use crate::theme::ORANGE;
use crate::theme::palette;

impl Gallery {
    pub(crate) fn overview(cx: &mut Context<Self>) -> impl IntoElement {
        let palette = palette(cx);
        let cards = SpinnerName::ALL.iter().map(|&name| {
            v_flex()
                .id(name.key())
                .w(px(208.))
                .h(px(208.))
                .p_1()
                .rounded_3xl()
                .bg(palette.subtle)
                .cursor_pointer()
                .hover(|style| style.bg(palette.raised))
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .mt_4()
                        .items_center()
                        .justify_center()
                        .child(AnySpinner::named(name, name.key()).size(px(24.))),
                )
                .child(
                    div()
                        .px_2()
                        .py_1p5()
                        .text_center()
                        .text_size(px(13.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(palette.muted)
                        .child(title(name)),
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.navigate(Page::Spinner(name), window, cx);
                }))
        });

        v_flex()
            .gap_6()
            .child(
                v_flex()
                    .gap_5()
                    .px_4()
                    .child(
                        div()
                            .text_size(px(36.))
                            .line_height(px(40.))
                            .font_weight(FontWeight::MEDIUM)
                            .child("Loading indicators")
                            .child(div().opacity(0.5).child("for GPUI.")),
                    )
                    .child(
                        div()
                            .text_color(palette.muted)
                            .child("A Rust port of the loading.dev spinners, drawn on the GPU."),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .h_12()
                    .pl_4()
                    .pr_3()
                    .rounded_2xl()
                    .bg(palette.subtle)
                    .border_1()
                    .border_color(cx.theme().border)
                    .font_family(palette.mono.clone())
                    .text_size(px(13.))
                    .text_color(palette.muted)
                    .child("$")
                    .child(
                        h_flex()
                            .flex_1()
                            .child("cargo add ")
                            .child(div().text_color(rgb(ORANGE)).child("gpui-loading")),
                    )
                    .child(Clipboard::new("copy-install").value("cargo add gpui-loading")),
            )
            .child(div().flex().flex_wrap().gap_2().children(cards))
    }
}
