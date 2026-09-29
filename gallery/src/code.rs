//! The preview's code, as the controls have it, with light highlighting.

use gpui_kit::App;
use gpui_kit::HighlightStyle;
use gpui_kit::StyledText;
use gpui_kit::component::Colorize;
use gpui_kit::component::clipboard::Clipboard;
use gpui_kit::component::h_flex;
use gpui_kit::component::v_flex;
use gpui_kit::div;
use gpui_kit::prelude::*;
use gpui_kit::px;
use gpui_kit::rgb;
use gpui_loading::SpinnerName;

use crate::gallery::Gallery;
use crate::spinner_page::SIZES;
use crate::spinner_page::millis;
use crate::theme::ORANGE;
use crate::theme::Palette;

impl Gallery {
    /// The code for the preview as the controls have it, one line of tokens
    /// per line of code.
    pub(crate) fn snippet(&self, name: SpinnerName, cx: &App) -> Vec<Vec<(Token, String)>> {
        use Token::Keyword;
        use Token::Number;
        use Token::Plain;
        use Token::Type;

        let component = name.component();
        let mut lines = vec![
            vec![
                (Keyword, "use".into()),
                (Plain, " gpui_loading::".into()),
                (Type, component.into()),
                (Plain, ";".into()),
            ],
            vec![],
            vec![
                (Type, component.into()),
                (Plain, "::new(".into()),
                (Token::String, format!("\"{}\"", name.key())),
                (Plain, ")".into()),
            ],
            vec![
                (Plain, "    .size(px(".into()),
                (Number, format!("{:.0}.", SIZES[self.size].1)),
                (Plain, "))".into()),
            ],
        ];
        let ms = millis(self.duration);
        if (ms - millis(name.default_duration())).abs() >= 1.0 {
            lines.push(vec![
                (Plain, "    .duration(".into()),
                (Type, "Duration".into()),
                (Plain, "::from_millis(".into()),
                (Number, format!("{ms:.0}")),
                (Plain, "))".into()),
            ]);
        }
        if let Some(color) = self.color(cx) {
            let hex = color.to_hex();
            lines.push(vec![
                (Plain, "    .color(rgb(".into()),
                (
                    Number,
                    format!("0x{}", hex.trim_start_matches('#').to_lowercase()),
                ),
                (Plain, "))".into()),
            ]);
        }
        let opacity = self.opacity;
        if opacity < 1.0 {
            lines.push(vec![
                (Plain, "    .opacity(".into()),
                (Number, format!("{opacity:.2}")),
                (Plain, ")".into()),
            ]);
        }
        if self.paused {
            lines.push(vec![
                (Plain, "    .play_state(".into()),
                (Type, "PlayState".into()),
                (Plain, "::Paused)".into()),
            ]);
        }
        lines
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Token {
    Plain,
    Keyword,
    Type,
    Number,
    String,
}

pub(crate) fn code_block(lines: &[Vec<(Token, String)>], palette: &Palette) -> impl IntoElement {
    let text: String = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|(_, text)| text.as_str())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let rows = lines.iter().enumerate().map(|(index, tokens)| {
        let mut source = String::new();
        let mut highlights = Vec::new();
        for (token, text) in tokens {
            let color = match token {
                Token::Plain => None,
                Token::Keyword => Some(ORANGE),
                Token::Type => Some(0x0093_53cc),
                Token::Number => Some(0x0002_bc83),
                Token::String => Some(0x001e_d6ff),
            };
            let range = source.len()..source.len() + text.len();
            source.push_str(text);
            if let Some(color) = color {
                highlights.push((
                    range,
                    HighlightStyle {
                        color: Some(rgb(color).into()),
                        ..HighlightStyle::default()
                    },
                ));
            }
        }
        h_flex()
            .h(px(20.))
            .child(
                div()
                    .w(px(32.))
                    .text_color(palette.muted)
                    .child((index + 1).to_string()),
            )
            .child(StyledText::new(source).with_highlights(highlights))
    });

    v_flex()
        .relative()
        .px_4()
        .py_3()
        .font_family(palette.mono.clone())
        .text_size(px(13.))
        .line_height(px(20.))
        .children(rows)
        .child(
            div()
                .absolute()
                .top_2()
                .right_2()
                .child(Clipboard::new("copy-code").value(text)),
        )
}
