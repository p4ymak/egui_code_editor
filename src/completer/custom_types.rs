use std::collections::{BTreeMap, HashMap};

/// Syntax style for method calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxStyle {
    /// Dot notation: self.move_to()
    Dot,
    /// Colon notation: self:move_to()
    Colon,
}

impl Default for SyntaxStyle {
    fn default() -> Self {
        Self::Dot
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompType {
    Global,
    Field,
    Function,
    Snippet,
}

/// Helper struct for building completions with a fluent API
pub struct CompletionsBuilder {
    items: Vec<(&'static str, &'static str, &'static str, CompType)>,
}

impl CompletionsBuilder {
    /// Create a new completions builder
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a new completion item
    pub fn add(&mut self, display: impl Into<String>, comp_type: CompType) -> ItemBuilder {
        ItemBuilder {
            builder: self,
            display: display.into(),
            comp_type,
            snippet: None,
            documentation: None,
        }
    }

    /// Finish building and return the completions
    pub fn build(self) -> Vec<(&'static str, &'static str, &'static str, CompType)> {
        self.items
    }
}

impl Default for CompletionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for a single completion item within CompletionsBuilder
pub struct ItemBuilder<'a> {
    builder: &'a mut CompletionsBuilder,
    display: String,
    comp_type: CompType,
    snippet: Option<String>,
    documentation: Option<String>,
}

impl<'a> ItemBuilder<'a> {
    /// Set the snippet to insert (can include $ for cursor position)
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }

    /// Set the documentation text
    pub fn with_docs(mut self, docs: impl Into<String>) -> Self {
        self.documentation = Some(docs.into());
        self
    }

    /// Finish this item and return the builder for adding more items
    pub fn done(self) -> &'a mut CompletionsBuilder {
        let display = Box::leak(self.display.clone().into_boxed_str());
        let snippet = Box::leak(
            self.snippet
                .unwrap_or_else(|| self.display.clone())
                .into_boxed_str(),
        );
        let docs = Box::leak(self.documentation.unwrap_or_default().into_boxed_str());

        self.builder
            .items
            .push((display, snippet, docs, self.comp_type));
        self.builder
    }
}

/// Trait for defining custom types with their completion information
/// Implement this trait on your types to provide autocomplete support
///
/// # Example
/// ```
/// struct MyCharacter;
///
/// impl CustomType for MyCharacter {
///     fn type_name() -> &'static str {
///         "self"
///     }
///     
///     fn build_completions(builder: &mut CompletionsBuilder) {
///         builder.add("move_to(..)", CompType::Function)
///             .with_snippet("move_to($)")
///             .with_docs("Moves the character")
///             .done();
///         
///         builder.add("get_health()", CompType::Function)
///             .with_snippet("get_health()")
///             .with_docs("Returns current health")
///             .done();
///     }
/// }
/// ```
pub trait CustomType {
    /// The name of the type (e.g., "self", "player", "world")
    fn type_name() -> &'static str;

    /// Build the list of completions for this type using the builder
    fn build_completions(builder: &mut CompletionsBuilder);

    /// The syntax style for this type (Dot or Colon)
    /// Defaults to Dot if not overridden
    fn syntax_style() -> SyntaxStyle {
        SyntaxStyle::Dot
    }
}

/// Represents a completion item with optional snippet and documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompletionItem {
    /// The full text to display and match against
    pub display: String,
    /// The snippet to insert when Tab is pressed (if None, uses display)
    pub snippet: Option<String>,
    /// Documentation to show in popup (supports markdown-like formatting)
    pub documentation: Option<String>,
    pub comp_type: CompType,
}

impl CompletionItem {
    pub fn new(display: impl Into<String>, comp_type: CompType) -> Self {
        Self {
            display: display.into(),
            snippet: None,
            documentation: None,
            comp_type,
        }
    }

    pub fn with_snippet(
        display: impl Into<String>,
        snippet: impl Into<String>,
        comp_type: CompType,
    ) -> Self {
        Self {
            display: display.into(),
            snippet: Some(snippet.into()),
            documentation: None,
            comp_type,
        }
    }

    pub fn with_snippet_and_docs(
        display: impl Into<String>,
        snippet: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) -> Self {
        Self {
            display: display.into(),
            snippet: Some(snippet.into()),
            documentation: Some(documentation.into()),
            comp_type,
        }
    }

    pub fn with_docs(
        display: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) -> Self {
        Self {
            display: display.into(),
            snippet: None,
            documentation: Some(documentation.into()),
            comp_type,
        }
    }

    /// Get the text to insert (snippet if available, otherwise display)
    pub fn insert_text(&self) -> &str {
        self.snippet.as_deref().unwrap_or(&self.display)
    }

    /// Check if this item has a cursor position marker ($)
    pub fn has_cursor_marker(&self) -> bool {
        self.insert_text().contains('$')
    }

    /// Get the cursor offset (position of $) and the text without $
    pub fn cursor_info(&self) -> (String, Option<usize>) {
        let text = self.insert_text();
        if let Some(pos) = text.find('$') {
            let without_marker = text.replace('$', "");
            (without_marker, Some(pos))
        } else {
            (text.to_string(), None)
        }
    }
}

/// Extension to the Completer for custom type support
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CustomTypeRegistry {
    /// Maps type names (like "self") to their available methods/properties
    pub types: HashMap<String, TypeInfo>,
    /// Global completions (not tied to a type)
    pub globals: BTreeMap<String, CompletionItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    pub items: BTreeMap<String, CompletionItem>,
    pub syntax_style: SyntaxStyle,
}

impl CustomTypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a type that implements the CustomType trait
    ///
    /// # Example
    /// ```
    /// registry.register_trait_type::<MyCharacter>();
    /// ```
    pub fn register_trait_type<T: CustomType>(&mut self) {
        let type_name = T::type_name();
        let mut builder = CompletionsBuilder::new();
        T::build_completions(&mut builder);
        let completions = builder.build();
        let syntax_style = T::syntax_style();
        self.register_type_with_snippets_and_style(type_name, completions, syntax_style);
    }

    /// Register a type with simple method names (no snippets)
    pub fn register_type_simple(&mut self, type_name: impl Into<String>, methods: Vec<String>) {
        let type_name = type_name.into();
        let methods_map = methods
            .into_iter()
            .map(|m| (m.clone(), CompletionItem::new(m, CompType::Function)))
            .collect();

        self.types.insert(
            type_name,
            TypeInfo {
                items: methods_map,
                syntax_style: SyntaxStyle::Dot,
            },
        );
    }

    /// Register a type with snippet and documentation support (uses Dot syntax by default)
    /// Each method is (name, snippet, docs, comp_type) where snippet can include $ for cursor position
    ///
    /// Example:
    /// ```
    /// registry.register_type_with_snippets(
    ///     "self",
    ///     vec![
    ///         ("move_to", "move_to($x, y)", "Moves the character to the specified position.\n\nParameters:\n- x: X coordinate\n- y: Y coordinate", CompType::Function),
    ///         ("set_health", "set_health($value)", "Sets the character's health.\n\nParameters:\n- value: New health value (0-100)", CompType::Function),
    ///     ],
    /// );
    /// ```
    pub fn register_type_with_snippets(
        &mut self,
        type_name: impl Into<String>,
        methods: Vec<(&str, &str, &str, CompType)>,
    ) {
        self.register_type_with_snippets_and_style(type_name, methods, SyntaxStyle::Dot);
    }

    /// Register a type with snippet and documentation support with explicit syntax style
    /// Each method is (name, snippet, docs, comp_type) where snippet can include $ for cursor position
    ///
    /// Example:
    /// ```
    /// registry.register_type_with_snippets_and_style(
    ///     "self",
    ///     vec![
    ///         ("move_to", "move_to($x, y)", "Moves the character", CompType::Function),
    ///     ],
    ///     SyntaxStyle::Colon,
    /// );
    /// ```
    pub fn register_type_with_snippets_and_style(
        &mut self,
        type_name: impl Into<String>,
        methods: Vec<(&str, &str, &str, CompType)>,
        syntax_style: SyntaxStyle,
    ) {
        let type_name = type_name.into();
        let methods_map = methods
            .into_iter()
            .map(|(name, snippet, docs, comp_type)| {
                (
                    name.to_string(),
                    CompletionItem::with_snippet_and_docs(name, snippet, docs, comp_type),
                )
            })
            .collect();

        self.types.insert(
            type_name,
            TypeInfo {
                items: methods_map,
                syntax_style,
            },
        );
    }

    /// Register a type with only snippets (no docs)
    /// Each method is (name, snippet, comp_type)
    ///
    /// Example:
    /// ```
    /// registry.register_type_snippets(
    ///     "self",
    ///     vec![
    ///         ("move_to", "move_to($x, y)", CompType::Function),
    ///         ("attack", "attack($target)", CompType::Function),
    ///     ],
    /// );
    /// ```
    pub fn register_type_snippets(
        &mut self,
        type_name: impl Into<String>,
        methods: Vec<(&str, &str, CompType)>,
    ) {
        let type_name = type_name.into();
        let methods_map = methods
            .into_iter()
            .map(|(name, snippet, comp_type)| {
                (
                    name.to_string(),
                    CompletionItem::with_snippet(name, snippet, comp_type),
                )
            })
            .collect();

        self.types.insert(
            type_name,
            TypeInfo {
                items: methods_map,
                syntax_style: SyntaxStyle::Dot,
            },
        );
    }

    /// Register a type with only documentation (no snippets)
    /// Each method is (name, docs, comp_type)
    ///
    /// Example:
    /// ```
    /// registry.register_type_docs(
    ///     "self",
    ///     vec![
    ///         ("move_to", "Moves the character", CompType::Function),
    ///         ("attack", "Attacks a target", CompType::Function),
    ///     ],
    /// );
    /// ```
    pub fn register_type_docs(
        &mut self,
        type_name: impl Into<String>,
        methods: Vec<(&str, &str, CompType)>,
    ) {
        let type_name = type_name.into();
        let methods_map = methods
            .into_iter()
            .map(|(name, docs, comp_type)| {
                (
                    name.to_string(),
                    CompletionItem::with_docs(name, docs, comp_type),
                )
            })
            .collect();

        self.types.insert(
            type_name,
            TypeInfo {
                items: methods_map,
                syntax_style: SyntaxStyle::Dot,
            },
        );
    }

    /// Register global completions (like 'foreach', 'if', etc.) with full options
    ///
    /// Example:
    /// ```
    /// // With snippet and docs
    /// registry.register_global(
    ///     "foreach",
    ///     Some("for $item in items {\n    \n}"),
    ///     Some("Iterates over each item in a collection."),
    ///     CompType::Global
    /// );
    ///
    /// // Just snippet, no docs
    /// registry.register_global("if", Some("if $condition {\n}"), None, CompType::Global);
    ///
    /// // Just docs, no snippet
    /// registry.register_global("self", None, Some("The character instance"), CompType::Field);
    /// ```
    pub fn register_global(
        &mut self,
        name: impl Into<String>,
        snippet: Option<impl Into<String>>,
        documentation: Option<impl Into<String>>,
        comp_type: CompType,
    ) {
        let name_str = name.into();
        let item = match (snippet, documentation) {
            (Some(s), Some(d)) => CompletionItem::with_snippet_and_docs(&name_str, s, d, comp_type),
            (Some(s), None) => CompletionItem::with_snippet(&name_str, s, comp_type),
            (None, Some(d)) => CompletionItem::with_docs(&name_str, d, comp_type),
            (None, None) => CompletionItem::new(&name_str, comp_type),
        };
        self.globals.insert(name_str, item);
    }

    /// Register a simple global without snippet or docs
    pub fn register_global_simple(&mut self, name: impl Into<String>, comp_type: CompType) {
        let name = name.into();
        self.globals
            .insert(name.clone(), CompletionItem::new(name, comp_type));
    }

    /// Register a global with only a snippet
    pub fn register_global_snippet(
        &mut self,
        name: impl Into<String>,
        snippet: impl Into<String>,
        comp_type: CompType,
    ) {
        let name_str = name.into();
        self.globals.insert(
            name_str.clone(),
            CompletionItem::with_snippet(&name_str, snippet, comp_type),
        );
    }

    /// Register a global with only documentation
    pub fn register_global_docs(
        &mut self,
        name: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) {
        let name_str = name.into();
        self.globals.insert(
            name_str.clone(),
            CompletionItem::with_docs(&name_str, documentation, comp_type),
        );
    }

    /// Register a global with snippet and documentation
    pub fn register_global_snippet_docs(
        &mut self,
        name: impl Into<String>,
        snippet: impl Into<String>,
        documentation: impl Into<String>,
        comp_type: CompType,
    ) {
        let name_str = name.into();
        self.globals.insert(
            name_str.clone(),
            CompletionItem::with_snippet_and_docs(&name_str, snippet, documentation, comp_type),
        );
    }

    /// Check if any registered type uses colon syntax
    pub fn has_colon_syntax(&self) -> bool {
        self.types
            .values()
            .any(|info| info.syntax_style == SyntaxStyle::Colon)
    }

    /// Get completions for a given prefix
    /// Returns (display_text, completion_item)
    pub fn get_completions(&self, prefix: &str) -> Vec<(String, CompletionItem)> {
        let mut results = Vec::new();

        // Check if we're completing a member access (e.g., "self.move" or "self:move")
        // Try both separators
        let separator_and_type = prefix
            .rsplit_once('.')
            .map(|(t, m)| (t, m, '.'))
            .or_else(|| prefix.rsplit_once(':').map(|(t, m)| (t, m, ':')));

        if let Some((type_part, method_prefix, separator)) = separator_and_type {
            let type_name = type_part.trim();

            if let Some(type_info) = self.types.get(type_name) {
                // Determine the correct separator for this type
                let correct_separator = match type_info.syntax_style {
                    SyntaxStyle::Dot => '.',
                    SyntaxStyle::Colon => ':',
                };

                // Add methods that match the prefix
                for (method_name, item) in &type_info.items {
                    if method_prefix.is_empty() || method_name.starts_with(method_prefix) {
                        let display = format!("{}{}{}", type_name, correct_separator, method_name);
                        results.push((display, item.clone()));
                    }
                }

                return results;
            }
        }

        // Check type names (e.g., "sel" -> "self")
        for type_name in self.types.keys() {
            if type_name.starts_with(prefix) {
                results.push((
                    type_name.clone(),
                    CompletionItem::new(type_name, CompType::Field),
                ));
            }
        }

        // Check globals
        for (name, item) in &self.globals {
            if name.starts_with(prefix) {
                results.push((name.clone(), item.clone()));
            }
        }

        results
    }
}
