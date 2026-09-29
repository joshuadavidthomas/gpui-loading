//! A copy of the loading.dev site: an overview of every spinner, and a page
//! for each with a live preview, the controls to tune it and the code for
//! what it shows. Built with gpui-kit's components, themed after the site.
//!
//! ```sh
//! cargo run -p gallery            # follows the system appearance
//! cargo run -p gallery -- --dark  # or --light
//! cargo run -p gallery -- --page bouncing-dots
//! ```

// Layout code reads best as one long builder chain.
#![allow(clippy::too_many_lines)]

mod code;
mod controls;
mod gallery;
mod options;
mod overview;
mod sidebar;
mod spinner_page;
mod spinners;
mod theme;

use gpui_kit::App;
use gpui_kit::TitlebarOptions;
use gpui_kit::WindowBounds;
use gpui_kit::WindowOptions;
use gpui_kit::assets::Assets;
use gpui_kit::prelude::*;
use gpui_kit::px;
use gpui_kit::size;
use gpui_loading::SpinnerAssets;
use gpui_loading::SpinnerName;

use crate::gallery::Gallery;
use crate::gallery::Page;
use crate::theme::install_theme;

fn main() {
    gpui_kit::application()
        .with_assets(SpinnerAssets::wrap(Assets))
        .run(|cx: &mut App| {
            gpui_kit::init(cx);

            let args: Vec<String> = std::env::args().collect();
            let pinned = if args.iter().any(|arg| arg == "--dark") {
                Some(true)
            } else if args.iter().any(|arg| arg == "--light") {
                Some(false)
            } else {
                None
            };
            install_theme(pinned.unwrap_or_default(), cx);

            let start = args
                .iter()
                .position(|arg| arg == "--page")
                .and_then(|index| args.get(index + 1))
                .and_then(|key| SpinnerName::from_key(key))
                .map_or(Page::Overview, Page::Spinner);

            let window = gpui_kit::open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::centered(size(px(1280.), px(860.)), cx)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("gpui-loading".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                cx,
                |window, cx| cx.new(|cx| Gallery::new(start, pinned.is_none(), window, cx)),
            );
            if let Err(error) = window {
                eprintln!("failed to open the gallery window: {error:#}");
                cx.quit();
                return;
            }
            cx.activate(true);
        });
}
