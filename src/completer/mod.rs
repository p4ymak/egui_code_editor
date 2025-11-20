pub mod custom_types;
mod trie;
use crate::{CodeEditor, ColorTheme, Syntax, Token, TokenType, format_token};
use custom_types::{CompType, CompletionItem, CustomTypeRegistry};
use egui::{Event, Frame, Modifiers, Sense, Stroke, TextBuffer, text_edit::TextEditOutput};
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
pub struct Completer {
    prefix: String,
    prefix_range: (usize, usize), // Start and end position of the prefix being completed
    cursor: usize,
    ignore_cursor: Option<usize>,
    trie_syntax: Trie,
    trie_user: Option<Trie>,
    variant_id: usize,
    completions: Vec<(String, CompletionItem)>, // Changed to Vec to maintain order and store items
    custom_types: CustomTypeRegistry,
}

impl Completer {
    /// Completer should be stored somewhere in your App struct.
    pub fn new_with_syntax(syntax: &Syntax) -> Self {
        Completer {
            trie_syntax: Trie::from(syntax),
            ..Default::default()
        }
    }

    pub fn with_user_words(self) -> Self {
        Completer {
            trie_user: Some(Trie::default()),
            ..self
        }
    }

    // Register a type that implements the CustomType trait (builder pattern)
    /// This is the recommended way to register custom types
    ///
    /// # Example
    /// ```rust
    /// let completer = Completer::new_with_syntax(&Syntax::rust())
    ///     .with_trait_type::<MyCharacter>();
    /// ```
    pub fn with_trait_type<T: custom_types::CustomType>(mut self) -> Self {
        self.custom_types.register_trait_type::<T>();
        self
    }

    /// Register a type that implements the CustomType trait on an existing completer
    ///
    /// # Example
    /// ```rust
    /// completer.register_trait_type::<MyCharacter>();
    /// ```
    pub fn register_trait_type<T: custom_types::CustomType>(&mut self) {
        self.custom_types.register_trait_type::<T>();
    }

    pub fn custom_types(&self) -> &CustomTypeRegistry {
        &self.custom_types
    }

    /// Register a custom type with simple method names
    pub fn with_custom_type(mut self, type_name: impl Into<String>, items: Vec<String>) -> Self {
        self.custom_types.register_type_simple(type_name, items);
        self
    }

    /// Register a custom type with snippet and documentation support
    ///
    /// # Example
    /// ```
    /// let completer = Completer::new_with_syntax(&Syntax::rust())
    ///     .with_custom_type_snippets_docs(
    ///         "self",
    ///         vec![
    ///             ("move_to", "move_to($x, y)", "Moves character to position", CompType::Function),
    ///             ("get_health", "get_health()", "Returns current health", CompType::Function),
    ///         ],
    ///     );
    /// ```
    pub fn with_custom_type_snippets_docs(
        mut self,
        type_name: impl Into<String>,
        methods: Vec<(&str, &str, &str, CompType)>,
    ) -> Self {
        self.custom_types
            .register_type_with_snippets(type_name, methods);
        self
    }

    /// Register a custom type with only snippets (no docs)
    ///
    /// # Example
    /// ```
    /// let completer = Completer::new_with_syntax(&Syntax::rust())
    ///     .with_custom_type_snippets(
    ///         "self",
    ///         vec![
    ///             ("move_to", "move_to($x, y)", CompType::Function),
    ///             ("attack", "attack($target)", CompType::Function),
    ///         ],
    ///     );
    /// ```
    pub fn with_custom_type_snippets(
        mut self,
        type_name: impl Into<String>,
        items: Vec<(&str, &str, CompType)>,
    ) -> Self {
        self.custom_types.register_type_snippets(type_name, items);
        self
    }

    /// Register a custom type with only documentation (no snippets)
    ///
    /// # Example
    /// ```
    /// let completer = Completer::new_with_syntax(&Syntax::rust())
    ///     .with_custom_type_docs(
    ///         "self",
    ///         vec![
    ///             ("move_to", "Moves character", CompType::Function),
    ///             ("attack", "Attacks target", CompType::Function),
    ///         ],
    ///     );
    /// ```
    pub fn with_custom_type_docs(
        mut self,
        type_name: impl Into<String>,
        items: Vec<(&str, &str, CompType)>,
    ) -> Self {
        self.custom_types.register_type_docs(type_name, items);
        self
    }

    /// Register a global snippet with optional documentation
    ///
    /// # Example
    /// ```
    /// // With docs
    /// let completer = Completer::new_with_syntax(&Syntax::rust())
    ///     .with_global("foreach", Some("for $item in items {\n}"), Some("Iterate over items"), CompType::Global);
    ///
    /// // Just snippet
    /// let completer = Completer::new_with_syntax(&Syntax::rust())
    ///     .with_global("if", Some("if $condition {\n}"), None, CompType::Global);
    /// ```
    pub fn with_global(
        mut self,
        name: impl Into<String>,
        snippet: Option<impl Into<String>>,
        documentation: Option<impl Into<String>>,
        comp_type: CompType,
    ) -> Self {
        self.custom_types
            .register_global(name, snippet, documentation, comp_type);
        self
    }

    /// Register a simple global (no snippet, no docs)
    pub fn with_global_simple(mut self, name: impl Into<String>, comp_type: CompType) -> Self {
        self.custom_types.register_global_simple(name, comp_type);
        self
    }

    /// Register a global with only a snippet
    pub fn with_global_snippet(
        mut self,
        name: impl Into<String>,
        snippet: impl Into<String>,
        comp_type: CompType,
    ) -> Self {
        self.custom_types
            .register_global_snippet(name, snippet, comp_type);
        self
    }

    /// Register a global with only documentation
    pub fn with_global_docs(
        mut self,
        name: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) -> Self {
        self.custom_types
            .register_global_docs(name, documentation, comp_type);
        self
    }

    /// Register a global with snippet and documentation
    pub fn with_global_snippet_docs(
        mut self,
        name: impl Into<String>,
        snippet: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) -> Self {
        self.custom_types
            .register_global_snippet_docs(name, snippet, documentation, comp_type);
        self
    }

    /// Add a custom type to an existing completer
    pub fn register_custom_type(&mut self, type_name: impl Into<String>, item: Vec<String>) {
        self.custom_types.register_type_simple(type_name, item);
    }

    /// Register a custom type with snippets and docs to an existing completer
    pub fn register_custom_type_snippets_docs(
        &mut self,
        type_name: impl Into<String>,
        items: Vec<(&str, &str, &str, CompType)>,
    ) {
        self.custom_types
            .register_type_with_snippets(type_name, items);
    }

    /// Register a custom type with only snippets
    pub fn register_custom_type_snippets(
        &mut self,
        type_name: impl Into<String>,
        items: Vec<(&str, &str, CompType)>,
    ) {
        self.custom_types.register_type_snippets(type_name, items);
    }

    /// Register a custom type with only docs
    pub fn register_custom_type_docs(
        &mut self,
        type_name: impl Into<String>,
        items: Vec<(&str, &str, CompType)>,
    ) {
        self.custom_types.register_type_docs(type_name, items);
    }

    /// Register a global snippet to an existing completer (flexible)
    pub fn register_global(
        &mut self,
        name: impl Into<String>,
        snippet: Option<impl Into<String>>,
        documentation: Option<impl Into<String>>,
        comp_type: CompType,
    ) {
        self.custom_types
            .register_global(name, snippet, documentation, comp_type);
    }

    /// Register a simple global
    pub fn register_global_simple(&mut self, name: impl Into<String>, comp_type: CompType) {
        self.custom_types.register_global_simple(name, comp_type);
    }

    /// Register a global with only a snippet
    pub fn register_global_snippet(
        &mut self,
        name: impl Into<String>,
        snippet: impl Into<String>,
        comp_type: CompType,
    ) {
        self.custom_types
            .register_global_snippet(name, snippet, comp_type);
    }

    /// Register a global with only docs
    pub fn register_global_docs(
        &mut self,
        name: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) {
        self.custom_types
            .register_global_docs(name, documentation, comp_type);
    }

    /// Register a global with snippet and docs
    pub fn register_global_snippet_docs(
        &mut self,
        name: impl Into<String>,
        snippet: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) {
        self.custom_types
            .register_global_snippet_docs(name, snippet, documentation, comp_type);
    }

    pub fn push_word(&mut self, word: &str) {
        self.trie_syntax.push(word);
    }

    pub fn handle_input(&mut self, ctx: &egui::Context) {
        if self.prefix.is_empty() {
            return;
        }

        if let Some(cursor) = self.ignore_cursor
            && cursor == self.cursor
        {
            return;
        }

        // Get completions from trie (these return just suffixes)
        let completions_syntax = self.trie_syntax.find_completions(&self.prefix);
        let completions_user = self
            .trie_user
            .as_ref()
            .map(|t| t.find_completions(&self.prefix))
            .unwrap_or_default();

        // Convert trie completions to full words
        let trie_items: Vec<(String, CompletionItem)> = completions_syntax
            .into_iter()
            .chain(completions_user)
            .map(|suffix| {
                let full_word = format!("{}{}", self.prefix, suffix);
                (
                    full_word.clone(),
                    CompletionItem::new(full_word, CompType::Global),
                )
            })
            .collect();

        // Get custom type completions (these already return full items)
        let custom_items = self.custom_types.get_completions(&self.prefix);

        // Combine and deduplicate
        let mut all_completions: BTreeSet<String> = BTreeSet::new();
        let mut completion_map: std::collections::HashMap<String, CompletionItem> =
            std::collections::HashMap::new();

        for (display, item) in trie_items.into_iter().chain(custom_items) {
            if all_completions.insert(display.clone()) {
                completion_map.insert(display, item);
            }
        }

        // Convert to sorted vec
        self.completions = all_completions
            .into_iter()
            .map(|display| {
                let item = completion_map.remove(&display).unwrap();
                (display, item)
            })
            .collect();

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
            } else if i.consume_key(Modifiers::NONE, egui::Key::Tab)
                || i.consume_key(Modifiers::NONE, egui::Key::Enter)
            {
                if let Some((display, item)) = self.completions.get(self.variant_id) {
                    // Determine what to delete and what to insert
                    // Check for both dot and colon separators
                    let separator_split = display
                        .rsplit_once('.')
                        .or_else(|| display.rsplit_once(':'));

                    let (delete_count, insert_text) =
                        if let Some((type_part, _method_part)) = separator_split {
                            let prefix_split = self
                                .prefix
                                .rsplit_once('.')
                                .or_else(|| self.prefix.rsplit_once(':'));

                            if let Some((prefix_type, prefix_method)) = prefix_split {
                                // Delete only the partial method part after the separator
                                let delete = prefix_method.len();
                                let insert = if item.snippet.is_some() {
                                    item.insert_text().to_string()
                                } else {
                                    _method_part.to_string()
                                };
                                (delete, insert)
                            } else {
                                // Shouldn't happen, but fallback to replacing everything
                                let delete = self.prefix_range.1 - self.prefix_range.0;
                                (delete, display.clone())
                            }
                        } else {
                            // Regular completion (no separator), replace the entire prefix
                            let delete = self.prefix_range.1 - self.prefix_range.0;
                            let insert = item.insert_text().to_string();
                            (delete, insert)
                        };

                    // Calculate cursor offset if there's a $ marker
                    let (final_text, cursor_offset) = if insert_text.contains('$') {
                        let pos = insert_text.find('$').unwrap();
                        (insert_text.replace('$', ""), Some(pos))
                    } else {
                        (insert_text, None)
                    };

                    // Delete the partial text, then insert the completion
                    for _ in 0..delete_count {
                        i.events.push(Event::Key {
                            key: egui::Key::Backspace,
                            physical_key: None,
                            pressed: true,
                            repeat: false,
                            modifiers: Modifiers::NONE,
                        });
                    }

                    i.events.push(Event::Paste(final_text.clone()));

                    // If there's a cursor position, move back to it
                    if let Some(offset) = cursor_offset {
                        let move_back = final_text.len() - offset;
                        for _ in 0..move_back {
                            i.events.push(Event::Key {
                                key: egui::Key::ArrowLeft,
                                physical_key: None,
                                pressed: true,
                                repeat: false,
                                modifiers: Modifiers::NONE,
                            });
                        }
                    }
                }
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
            // Update Completer Dictionary
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

            if self.cursor != cursor.index {
                self.cursor = cursor.index;
                self.prefix.clear();
                self.completions.clear();
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

            // Enhanced prefix extraction that handles member access
            if next_char_allows {
                let text = galley.text();
                let text_before_cursor = text.char_range(0..cursor.index);

                // Find the start of the current completion context
                // Include ':' as a valid separator only if any registered type uses colon syntax
                let has_colon_syntax = self.custom_types.has_colon_syntax();

                let context_start = text_before_cursor
                    .rfind(|c: char| {
                        !c.is_alphanumeric()
                            && c != '_'
                            && c != '.'
                            && !(has_colon_syntax && c == ':')
                    })
                    .map(|pos| pos + 1)
                    .unwrap_or(0);

                // Find the first valid char boundary at or before context_start
                let safe_start = text_before_cursor
                    .char_indices()
                    .take_while(|(i, _)| *i <= context_start)
                    .last()
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                self.prefix = text_before_cursor[safe_start..].to_string();
                self.prefix_range = (safe_start, cursor.index);
            } else {
                self.prefix = String::new();
                self.prefix_range = (cursor.index, cursor.index);
            }

            if !(self.prefix.is_empty() || self.completions.is_empty()) {
                let completion_popup_response = egui::Popup::new(
                    egui::Id::new("Completer"),
                    ctx.clone(),
                    cursor_rect,
                    editor_output.response.layer_id,
                )
                .frame(Frame::popup(&ctx.style()).fill(theme.bg()))
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
                            for (i, (display, _item)) in self.completions.iter().enumerate() {
                                // Determine token type for coloring
                                let token_type = if display.contains('.') {
                                    TokenType::Function
                                } else if syntax.is_keyword(display) {
                                    TokenType::Keyword
                                } else if syntax.is_special(display) {
                                    TokenType::Special
                                } else if syntax.is_type(display) {
                                    TokenType::Type
                                } else {
                                    TokenType::Literal
                                };

                                let fmt = format_token(theme, fontsize, token_type, None);
                                let colored_text =
                                    egui::text::LayoutJob::single_section(display.clone(), fmt);

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
                    // Return the popup rect for positioning the docs popup
                    ui.min_rect()
                });

                // Show documentation popup to the right of the completion list
                if let Some(popup_response) = completion_popup_response {
                    let popup_rect = popup_response.inner;
                    if let Some((_display, item)) = self.completions.get(self.variant_id) {
                        if let Some(docs) = &item.documentation {
                            // Position docs popup to the right of completion popup
                            let docs_rect = egui::Rect::from_min_size(
                                egui::pos2(popup_rect.right() + 5.0, popup_rect.top()),
                                egui::vec2(1.0, 1.0), // Will auto-size
                            );

                            egui::Popup::new(
                                egui::Id::new("Completer_Docs"),
                                ctx.clone(),
                                docs_rect,
                                editor_output.response.layer_id,
                            )
                            .frame(Frame::popup(&ctx.style()).fill(theme.bg()))
                            .sense(Sense::empty())
                            .show(|ui| {
                                ui.response().sense = Sense::empty();
                                ui.set_max_width(300.0);
                                ui.set_max_height(400.0);

                                egui::ScrollArea::vertical()
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);

                                        let mut editor = CodeEditor::default()
                                            .readonly(true)
                                            .with_fontsize(14.0)
                                            .with_theme(theme.clone())
                                            .with_syntax(syntax.to_owned())
                                            .with_numlines(false);

                                        editor.show(ui, &mut docs.clone());
                                    });
                            });
                        }
                    }
                }
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
