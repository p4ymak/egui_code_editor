mod trie;

use crate::{ColorTheme, Syntax, Token, TokenType, format_token};
use egui::{
    Event, Frame, Modifiers, Sense, Stroke, TextBuffer, text::CCursor, text_edit::TextEditOutput,
    text_selection::text_cursor_state::ccursor_previous_word,
};
use std::collections::BTreeSet;
use trie::Trie;

impl From<&Syntax> for Trie {
    fn from(syntax: &Syntax) -> Trie {
        let mut trie = Trie::default();

        syntax.keywords.iter().for_each(|word| trie.push(word));
        syntax.types.iter().for_each(|word| trie.push(word));
        syntax.special.iter().for_each(|word| trie.push(word));
        if !syntax.case_sensitive {
            syntax
                .keywords
                .iter()
                .for_each(|word| trie.push(&word.to_lowercase()));
            syntax
                .types
                .iter()
                .for_each(|word| trie.push(&word.to_lowercase()));
            syntax
                .special
                .iter()
                .for_each(|word| trie.push(&word.to_lowercase()));
        }
        trie
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
/// Code-completer with pop-up above CodeEditor.
/// In future releases will be replaced with trait.
pub struct Completer {
    prefix: String,
    cursor: usize,
    indent: Option<String>,
    ignore_cursor: Option<usize>,
    trie_syntax: Trie,
    trie_user: Option<Trie>,
    variant_id: usize,
    completions: BTreeSet<String>,
}

impl Completer {
    /// Completer should be stored somewhere in your App struct.
    pub fn new_with_syntax(syntax: &Syntax) -> Self {
        Completer {
            trie_syntax: Trie::from(syntax),
            ..Default::default()
        }
    }
    /// Completer will preserve indentation for next lines.
    pub fn with_auto_indent(self) -> Self {
        Completer {
            indent: Some(String::new()),
            ..self
        }
    }
    /// Completer will have second dictionary for words besides Syntax.
    pub fn with_user_words(self) -> Self {
        Completer {
            trie_user: Some(Trie::default()),
            ..self
        }
    }
    pub fn push_word(&mut self, word: &str) {
        self.trie_syntax.push(word);
    }

    /// If using Completer without CodeEditor this method should be called before text-editing widget.
    /// Up/Down arrows for selection, Tab for completion, Esc for hiding
    pub fn handle_input(&mut self, ctx: &egui::Context) {
        if let Some(indent) = self.indent.as_mut()
            && !indent.is_empty()
        {
            ctx.input_mut(|i| {
                if i.consume_key(Modifiers::NONE, egui::Key::Enter) {
                    i.events
                        .push(Event::Paste(format!("\n{}", std::mem::take(indent))))
                }
            });
        }
        if self.prefix.is_empty() {
            return;
        }
        if let Some(cursor) = self.ignore_cursor
            && cursor == self.cursor
        {
            return;
        }

        let completions_syntax = self.trie_syntax.find_completions(&self.prefix);
        let completions_user = self
            .trie_user
            .as_ref()
            .map(|t| t.find_completions(&self.prefix))
            .unwrap_or_default();
        self.completions =
            BTreeSet::from_iter(completions_syntax.into_iter().chain(completions_user));
        if self.completions.is_empty() {
            return;
        }
        let last = self.completions.len().saturating_sub(1);
        ctx.input_mut(|i| {
            if i.consume_key(Modifiers::NONE, egui::Key::Escape) {
                self.ignore_cursor = Some(self.cursor);
            } else if i.consume_key(Modifiers::NONE, egui::Key::ArrowDown) {
                self.variant_id = if self.variant_id == last {
                    0
                } else {
                    self.variant_id.saturating_add(1).min(last)
                };
            } else if i.consume_key(Modifiers::NONE, egui::Key::ArrowUp) {
                self.variant_id = if self.variant_id == 0 {
                    last
                } else {
                    self.variant_id.saturating_sub(1)
                };
            } else if i.consume_key(Modifiers::NONE, egui::Key::Tab) {
                let completion = self
                    .completions
                    .iter()
                    .nth(self.variant_id)
                    .map(String::from)
                    .unwrap_or_default();
                i.events.push(Event::Paste(completion));
            }
        });
    }

    /// If using Completer without CodeEditor this method should be called after text-editing widget as it uses &mut TextEditOutput
    pub fn show(
        &mut self,
        syntax: &Syntax,
        theme: &ColorTheme,
        fontsize: f32,
        editor_output: &mut TextEditOutput,
    ) {
        if !editor_output.response.has_focus() {
            return;
        }
        let ctx = editor_output.response.ctx.clone();
        let galley = &editor_output.galley;

        if editor_output.response.changed() {
            // Update Competer Dictionary
            if let Some(trie_user) = self.trie_user.as_mut() {
                trie_user.clear();
                Token::default()
                    .tokens(syntax, galley.text())
                    .iter()
                    .filter(|t| matches!(t.ty(), TokenType::Literal | TokenType::Function))
                    .for_each(|t| trie_user.push(t.buffer()));
            }
        }

        // Auto-Completer
        let cursor_range = editor_output.state.cursor.char_range();
        if let Some(range) = cursor_range {
            let cursor = range.primary;
            let cursor_pos_in_galley = galley.pos_from_cursor(cursor);
            let cursor_rect =
                cursor_pos_in_galley.translate(editor_output.response.rect.left_top().to_vec2());
            // let cursor_on_screen = editor_output.response.rect.left_top()
            // + cursor_pos_in_galley.left_bottom().to_vec2();
            let word_start = ccursor_previous_word(galley.text(), cursor);
            if self.cursor != cursor.index {
                self.cursor = cursor.index;
                self.prefix.clear();
                // self.completions.clear();
                self.ignore_cursor = None;
                self.variant_id = 0;
            }

            if self.ignore_cursor.is_some_and(|c| c == self.cursor) {
                editor_output.response.request_focus();
                return;
            } else {
                self.ignore_cursor = None;
            }
            let next_char_allows = galley
                .chars()
                .nth(cursor.index)
                .is_none_or(|c| !(c.is_alphanumeric() || c == '_'))
                || (range.secondary.index > range.primary.index);

            // Preserve Line indentation
            if let Some(indent) = self.indent.as_mut() {
                let line_start = find_line_start_saturated(galley.text(), cursor);
                *indent = galley
                    .text()
                    .char_range(line_start.index..cursor.index)
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect();
            }

            self.prefix = if next_char_allows {
                let prefix = galley
                    .text()
                    .char_range(word_start.index..cursor.index)
                    .to_string();
                if let Some((_, tail)) =
                    prefix.rsplit_once(|c: char| !(c.is_alphanumeric() || c == '_'))
                {
                    tail.to_string()
                } else {
                    prefix
                }
            } else {
                String::new()
            };
            if !(self.prefix.is_empty() || self.completions.is_empty()) {
                egui::Popup::new(
                    egui::Id::new("Completer"),
                    ctx.clone(),
                    cursor_rect,
                    editor_output.response.layer_id,
                )
                .frame(Frame::popup(&ctx.global_style()).fill(theme.bg()))
                .sense(Sense::empty())
                .show(|ui| {
                    ui.response().sense = Sense::empty();
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    let height = (fontsize
                        + ui.style().visuals.widgets.hovered.bg_stroke.width * 2.0
                        + ui.style().spacing.button_padding.y * 2.0
                        + ui.style().spacing.item_spacing.y)
                        * self.completions.len().min(10) as f32
                        - ui.style().spacing.item_spacing.y;
                    ui.set_height(height);

                    egui::ScrollArea::vertical()
                        .auto_shrink([true, true])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            for (i, completion) in self.completions.iter().enumerate() {
                                let word = format!("{}{completion}", &self.prefix);
                                let token_type = match &word {
                                    word if syntax.is_keyword(word) => TokenType::Keyword,
                                    word if syntax.is_special(word) => TokenType::Special,
                                    word if syntax.is_type(word) => TokenType::Type,
                                    _ => TokenType::Literal,
                                };
                                let fmt = format_token(theme, fontsize, token_type);
                                let colored_text = egui::text::LayoutJob::single_section(word, fmt);
                                let selected = i == self.variant_id;

                                let button = ui.add(
                                    egui::Button::new(colored_text)
                                        .sense(Sense::empty())
                                        .frame(true)
                                        .fill(theme.bg())
                                        .stroke(if selected {
                                            Stroke::new(
                                                ui.style().visuals.widgets.hovered.bg_stroke.width,
                                                theme.type_color(TokenType::Literal),
                                            )
                                        } else {
                                            Stroke::NONE
                                        }),
                                );
                                if selected {
                                    button.scroll_to_me(None);
                                }
                            }
                        });
                });
            }
        }
    }

    /// Completer on text-editing widget, see demo for example
    pub fn show_on_text_widget(
        &mut self,
        ui: &mut egui::Ui,
        syntax: &Syntax,
        theme: &ColorTheme,
        mut widget: impl FnMut(&mut egui::Ui) -> TextEditOutput,
    ) -> TextEditOutput {
        self.handle_input(ui.ctx());
        let fontsize = ui.text_style_height(&egui::TextStyle::Monospace);
        let mut output = widget(ui);
        self.show(syntax, theme, fontsize, &mut output);
        output
    }
}

pub fn find_line_start_saturated(text: &str, current_index: CCursor) -> CCursor {
    let chars_count = text.chars().count();

    let position = text
        .chars()
        .rev()
        .skip(chars_count.saturating_sub(current_index.index))
        .position(|x| x == '\n');

    match position {
        Some(pos) => CCursor::new(current_index.index.saturating_sub(pos)),
        None => CCursor::new(0),
    }
}
