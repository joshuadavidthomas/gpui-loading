//! The site's look: its palette, its light and dark themes, and slimmer
//! scrollbars than gpui-kit draws.

use gpui_kit::App;
use gpui_kit::Hsla;
use gpui_kit::SharedString;
use gpui_kit::Window;
use gpui_kit::base::ScrollbarThumbStyle;
use gpui_kit::base::ScrollbarTrackStyle;
use gpui_kit::base::Theme as BaseTheme;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::Theme;
use gpui_kit::component::ThemeMode;
use gpui_kit::component::ThemeRegistry;
use gpui_kit::px;

pub(crate) const ORANGE: u32 = 0x00ef_551a;

pub(crate) const THEMES: &str = include_str!("theme.json");
pub(crate) const LIGHT_THEME: &str = "loading.dev Light";
pub(crate) const DARK_THEME: &str = "loading.dev Dark";

/// The site's tints, as the theme names them.
///
/// Rounded corners and borders together are the priciest thing GPUI's quad
/// shader draws, so the many cards have only the one: they stand out from
/// the page by colour instead of by outline.
pub(crate) struct Palette {
    /// The page itself, and the preview.
    pub(crate) surface: Hsla,
    /// `background-subtle`: the sidebar, the cards, the preview's frame.
    pub(crate) subtle: Hsla,
    /// `background`: a step up from what it sits on. It's translucent, so
    /// stacking it steps up again: the panel, its controls, their fills.
    pub(crate) raised: Hsla,
    pub(crate) border: Hsla,
    pub(crate) text: Hsla,
    pub(crate) muted: Hsla,
    pub(crate) mono: SharedString,
}

pub(crate) fn palette(cx: &App) -> Palette {
    let theme = cx.theme();
    Palette {
        surface: theme.background,
        subtle: theme.sidebar,
        raised: theme.secondary,
        border: theme.border,
        text: theme.foreground,
        muted: theme.muted_foreground,
        mono: theme.mono_font_family.clone(),
    }
}

/// The site has no theme switch: it follows the system, and so does the
/// gallery unless `--light` or `--dark` pins one.
pub(crate) fn apply_appearance(window: &mut Window, cx: &mut App) {
    Theme::sync_system_appearance(Some(window), cx);
    restyle_scrollbars(cx);
}

/// Registers the site's themes, switching to the one for `dark`. The window
/// takes over from the system appearance when it opens, unless pinned.
pub(crate) fn install_theme(dark: bool, cx: &mut App) {
    let registry = ThemeRegistry::global_mut(cx);
    if let Err(error) = registry.load_themes_from_str(THEMES) {
        eprintln!("failed to load the gallery's theme: {error:#}");
        return;
    }
    let themes = registry.themes();
    let (Some(light), Some(dark_theme)) = (
        themes.get(LIGHT_THEME).cloned(),
        themes.get(DARK_THEME).cloned(),
    ) else {
        return;
    };
    Theme::update(cx, |theme| {
        theme.light_theme = light;
        theme.dark_theme = dark_theme;
    });
    Theme::change(
        if dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        },
        None,
        cx,
    );
    restyle_scrollbars(cx);
}

/// gpui-kit's scrollbar sits in a square 16px track and widens from 6px to
/// 8px on hover. The site's is a slim rounded overlay that stays put.
/// gpui-kit rebuilds this with every theme change, so reapply it after one.
pub(crate) fn restyle_scrollbars(cx: &mut App) {
    let base = BaseTheme::global_mut(cx);
    let thumb = |style: ScrollbarThumbStyle| style.width(px(6.)).inset(px(3.)).radius(px(3.));
    let track = |_| ScrollbarTrackStyle::default().width(px(12.));
    let styles = base
        .scrollbar
        .styles()
        .clone()
        .track(track)
        .track_hover(track)
        .track_active(track)
        .thumb(thumb)
        .thumb_hover(thumb)
        .thumb_active(thumb);
    base.scrollbar = base.scrollbar.clone().with_styles(styles);
}
