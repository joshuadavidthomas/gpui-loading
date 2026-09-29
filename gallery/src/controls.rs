//! The site's controls, drawn by hand: gpui-kit's look differently.

use std::cell::Cell;
use std::rc::Rc;

use gpui_kit::Bounds;
use gpui_kit::Context;
use gpui_kit::MouseButton;
use gpui_kit::MouseDownEvent;
use gpui_kit::MouseMoveEvent;
use gpui_kit::Pixels;
use gpui_kit::canvas;
use gpui_kit::component::h_flex;
use gpui_kit::div;
use gpui_kit::prelude::*;
use gpui_kit::px;
use gpui_kit::relative;

use crate::gallery::Gallery;
use crate::theme::Palette;

/// A borderless square button holding an icon, as the site's pause button.
pub(crate) fn icon_button(
    id: &'static str,
    palette: &Palette,
) -> gpui_kit::Stateful<gpui_kit::Div> {
    div()
        .id(id)
        .flex()
        .flex_none()
        .size_9()
        .items_center()
        .justify_center()
        .rounded_lg()
        .text_color(palette.muted)
        .cursor_pointer()
        .hover(|style| style.bg(palette.raised).text_color(palette.text))
}

/// The site's slider: a bar filled to `fraction`, its label and value
/// inside it. Pressing or dragging along it calls `on_change` with the
/// fraction under the pointer.
pub(crate) fn slider(
    id: &'static str,
    label: &'static str,
    value: String,
    fraction: f64,
    palette: &Palette,
    cx: &mut Context<Gallery>,
    on_change: impl Fn(&mut Gallery, f64) + 'static,
) -> impl IntoElement {
    let bounds: Rc<Cell<Bounds<Pixels>>> = Rc::default();
    let on_change = Rc::new(on_change);
    let fraction_at = {
        let bounds = Rc::clone(&bounds);
        move |x: Pixels| {
            let bounds = bounds.get();
            f64::from((x - bounds.left()) / bounds.size.width).clamp(0.0, 1.0)
        }
    };
    #[allow(clippy::cast_possible_truncation)]
    let fill = fraction.clamp(0.0, 1.0) as f32;

    div()
        .id(id)
        .group(id)
        .relative()
        .h_8()
        .rounded_lg()
        .overflow_hidden()
        .bg(palette.raised)
        .cursor_ew_resize()
        .child(
            canvas(
                {
                    let bounds = Rc::clone(&bounds);
                    move |measured, _, _| bounds.set(measured)
                },
                |_, (), _, _| {},
            )
            .absolute()
            .size_full(),
        )
        .child(
            div()
                .absolute()
                .left_0()
                .top_0()
                .h_full()
                .w(relative(fill))
                .bg(palette.raised),
        )
        .child(
            h_flex()
                .absolute()
                .inset_0()
                .px_2()
                .justify_between()
                .text_color(palette.muted)
                .group_hover(id, |style| style.text_color(palette.text))
                .child(label)
                .child(
                    div()
                        .font_family(palette.mono.clone())
                        .text_size(px(12.))
                        .child(value),
                ),
        )
        .on_mouse_down(MouseButton::Left, {
            let on_change = Rc::clone(&on_change);
            let fraction_at = fraction_at.clone();
            cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                on_change(this, fraction_at(event.position.x));
                cx.notify();
            })
        })
        .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
            if event.pressed_button == Some(MouseButton::Left) {
                on_change(this, fraction_at(event.position.x));
                cx.notify();
            }
        }))
}
