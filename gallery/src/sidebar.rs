//! The sidebar: a home link, Overview, every spinner, and links out.

use gpui_kit::ClickEvent;
use gpui_kit::Context;
use gpui_kit::FontWeight;
use gpui_kit::SharedString;
use gpui_kit::Window;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::IconName;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::sidebar::SidebarItem;
use gpui_kit::component::sidebar::SidebarMenu;
use gpui_kit::component::sidebar::SidebarMenuItem;
use gpui_kit::component::v_flex;
use gpui_kit::div;
use gpui_kit::prelude::*;
use gpui_kit::px;
use gpui_kit::relative;
use gpui_loading::SpinnerName;

use crate::gallery::Gallery;
use crate::gallery::Page;
use crate::spinners::title;
use crate::theme::palette;

pub(crate) const REPOSITORY: &str = "https://github.com/joshuadavidthomas/gpui-loading";

/// A sidebar entry, its text inset as on the site. gpui-kit fixes its height
/// at 28px, so lists space them 6px apart to keep the site's 34px rhythm.
pub(crate) fn nav_item(label: impl Into<SharedString>) -> SidebarMenuItem {
    SidebarMenuItem::new(label).px_3()
}

/// Where `name` sits in the sidebar's list of spinners.
pub(crate) fn nav_index(name: SpinnerName) -> usize {
    SpinnerName::ALL
        .iter()
        .position(|&each| each == name)
        .unwrap_or_default()
}

impl Gallery {
    /// gpui-kit's menu items in a sidebar of our own: gpui-kit's `Sidebar`
    /// keeps its scroll position to itself, and the list here has to scroll
    /// to the open page's spinner.
    pub(crate) fn sidebar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = palette(cx);
        let theme = cx.theme();
        let (background, foreground, border) = (
            theme.sidebar,
            theme.sidebar_foreground,
            theme.sidebar_border,
        );

        // One child per spinner, so the scroll handle can find each by index.
        let mut spinners = Vec::with_capacity(SpinnerName::ALL.len());
        for (index, &name) in SpinnerName::ALL.iter().enumerate() {
            let item = nav_item(title(name))
                .active(self.page == Page::Spinner(name))
                .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.navigate(Page::Spinner(name), window, cx);
                }));
            spinners.push(item.render(("nav", index), window, cx).into_any_element());
        }

        let overview = SidebarMenu::new()
            .gap_1p5()
            .child(
                nav_item("Overview")
                    .active(self.page == Page::Overview)
                    .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                        this.navigate(Page::Overview, window, cx);
                    })),
            )
            .render("overview", window, cx)
            .into_any_element();

        let links = SidebarMenu::new()
            .gap_1p5()
            .child(
                nav_item("GitHub")
                    .icon(IconName::Github)
                    .on_click(|_, _, cx| cx.open_url(REPOSITORY)),
            )
            .child(
                nav_item("loading.dev")
                    .icon(IconName::Globe)
                    .on_click(|_, _, cx| cx.open_url("https://loading.dev")),
            )
            .render("links", window, cx)
            .into_any_element();

        // Spacing follows the site: sections 24px apart, the logo inset
        // 24px, everything else 16px, and Overview 20px above the spinners.
        v_flex()
            .flex_none()
            .w(px(256.))
            .h_full()
            .gap_6()
            .bg(background)
            .text_color(foreground)
            .border_r_1()
            .border_color(border)
            .child(
                div().px_6().pt_6().pb_2().child(
                    div()
                        .id("home")
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(cx.theme().foreground)
                        .cursor_pointer()
                        .child("gpui-loading")
                        .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                            this.navigate(Page::Overview, window, cx);
                        })),
                ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .gap_5()
                    .child(div().px_4().child(overview))
                    .child(
                        div()
                            .id("nav-area")
                            .relative()
                            .flex_1()
                            .min_h_0()
                            .child(
                                v_flex()
                                    .id("nav")
                                    .size_full()
                                    .overflow_y_scroll()
                                    .track_scroll(&self.nav_scroll)
                                    .gap_1p5()
                                    .px_4()
                                    .children(spinners),
                            )
                            .vertical_scrollbar(&self.nav_scroll),
                    ),
            )
            .child(
                v_flex()
                    .child(div().px_4().pb_4().child(links))
                    .child(
                        v_flex()
                            .gap_6()
                            .px_4()
                            .pb_4()
                            .child(div().h(px(1.)).bg(border))
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .line_height(relative(1.625))
                                    .text_color(palette.muted)
                                    .child(
                                        "Spinners designed by Jakub Krehel and Paul Faivret for loading.dev.",
                                    ),
                            ),
                    ),
            )
    }
}
