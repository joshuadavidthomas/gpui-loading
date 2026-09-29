//! The app: which page is open, the preview's settings, and the layout
//! around them.

use std::time::Duration;

use gpui_kit::AnyElement;
use gpui_kit::App;
use gpui_kit::Context;
use gpui_kit::ElementId;
use gpui_kit::Entity;
use gpui_kit::Hsla;
use gpui_kit::ScrollHandle;
use gpui_kit::Subscription;
use gpui_kit::Window;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::color_picker::ColorPickerEvent;
use gpui_kit::component::color_picker::ColorPickerState;
use gpui_kit::component::h_flex;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::div;
use gpui_kit::prelude::*;
use gpui_kit::px;
use gpui_loading::SpinnerName;

use crate::sidebar::nav_index;
use crate::theme::apply_appearance;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Page {
    Overview,
    Spinner(SpinnerName),
}

pub(crate) struct Gallery {
    pub(crate) page: Page,
    pub(crate) size: usize,
    pub(crate) paused: bool,
    pub(crate) panel_open: bool,
    pub(crate) duration: Duration,
    pub(crate) opacity: f32,
    pub(crate) color: Entity<ColorPickerState>,
    /// Paint with the inherited text colour, which the picker then shows,
    /// until a colour is picked.
    pub(crate) color_inherits: bool,
    /// The sidebar's list of spinners.
    pub(crate) nav_scroll: ScrollHandle,
    pub(crate) _subscriptions: Vec<Subscription>,
}

impl Gallery {
    pub(crate) fn new(
        page: Page,
        follow_system: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Gallery {
        let color = cx.new(|cx| ColorPickerState::new(window, cx));
        // Only a pick emits a change; setting the value doesn't.
        let mut subscriptions = vec![cx.subscribe(&color, |this, _, _: &ColorPickerEvent, cx| {
            this.color_inherits = false;
            cx.notify();
        })];
        if follow_system {
            apply_appearance(window, cx);
            subscriptions.push(cx.observe_window_appearance(window, |_, window, cx| {
                apply_appearance(window, cx);
            }));
        }
        let mut gallery = Gallery {
            page: Page::Overview,
            size: 1,
            paused: false,
            panel_open: true,
            duration: Duration::ZERO,
            opacity: 1.0,
            color,
            color_inherits: true,
            nav_scroll: ScrollHandle::new(),
            _subscriptions: subscriptions,
        };
        gallery.navigate(page, window, cx);
        if let Page::Spinner(name) = page {
            // The list has no layout before it first paints, and the scroll
            // handle drops a request made before then, so ask again after.
            let nav_scroll = gallery.nav_scroll.clone();
            window.on_next_frame(move |window, _| {
                nav_scroll.scroll_to_item(nav_index(name));
                window.refresh();
            });
        }
        gallery
    }

    /// Opens `page`, its controls at their defaults.
    pub(crate) fn navigate(&mut self, page: Page, window: &mut Window, cx: &mut Context<Self>) {
        self.page = page;
        if let Page::Spinner(name) = page {
            self.reset(name, window, cx);
            self.nav_scroll.scroll_to_item(nav_index(name));
        }
        cx.notify();
    }

    pub(crate) fn reset(&mut self, name: SpinnerName, window: &mut Window, cx: &mut Context<Self>) {
        self.size = 1;
        self.paused = false;
        self.duration = name.default_duration();
        self.opacity = 1.0;
        self.color_inherits = true;
        self.sync_inherited_color(window, cx);
    }

    /// Shows the text colour in the picker while the spinner inherits it,
    /// following the theme when that changes.
    pub(crate) fn sync_inherited_color(&self, window: &mut Window, cx: &mut Context<Self>) {
        let text = cx.theme().foreground;
        if self.color_inherits && self.color.read(cx).value() != Some(text) {
            self.color
                .update(cx, |state, cx| state.set_value(text, window, cx));
        }
    }

    /// The colour to paint with, or `None` to inherit the text colour.
    pub(crate) fn color(&self, cx: &App) -> Option<Hsla> {
        if self.color_inherits {
            None
        } else {
            self.color.read(cx).value()
        }
    }
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_inherited_color(window, cx);
        let (page, key): (AnyElement, &str) = match self.page {
            Page::Overview => (Self::overview(cx).into_any_element(), "overview"),
            Page::Spinner(name) => (self.spinner_page(name, cx).into_any_element(), name.key()),
        };

        h_flex()
            .size_full()
            .items_start()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.sidebar(window, cx))
            .child(
                // Keyed by page, so each page opens scrolled to the top.
                div()
                    .id(ElementId::Name(format!("content-{key}").into()))
                    .flex_1()
                    .h_full()
                    .overflow_y_scrollbar()
                    .child(
                        h_flex()
                            .justify_center()
                            .px_6()
                            .pt(px(80.))
                            .pb(px(80.))
                            .child(div().w(px(640.)).child(page)),
                    ),
            )
    }
}
