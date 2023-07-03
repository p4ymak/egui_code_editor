use super::syntax::TokenType;
use super::CodeEditor;
use egui::text::LayoutJob;

pub fn highlight(ctx: &egui::Context, cache: &CodeEditor, text: &str) -> LayoutJob {
    ctx.memory_mut(|mem| mem.caches.cache::<HighlightCache>().get((cache, text)))
}

#[derive(Default)]
pub struct Highlighter {}
impl egui::util::cache::ComputerMut<(&CodeEditor, &str), LayoutJob> for Highlighter {
    fn compute(&mut self, (cache, text): (&CodeEditor, &str)) -> LayoutJob {
        self.highlight(cache, text)
    }
}

pub type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

impl Highlighter {
    pub fn highlight(&self, editor: &CodeEditor, text: &str) -> LayoutJob {
        let mut job = LayoutJob::default();
        let mut text = text;
        while !text.is_empty() {
            // Comment
            if text.starts_with(editor.syntax.comment()) {
                let end = text.find('\n').unwrap_or(text.len());
                job.append(&text[..end], 0.0, editor.format(TokenType::Comment));
                text = &text[end..];
            }
            // Numeric
            else if text.starts_with(|c: char| c.is_numeric()) {
                let end = text[1..]
                    .find(|c: char| !c.is_numeric())
                    .map_or_else(|| text.len(), |i| i + 1);
                let word = &text[..end];
                job.append(word, 0.0, editor.format(TokenType::Numeric));
                text = &text[end..];
            }
            // String
            else if text.starts_with('\"') {
                let end = text[1..]
                    .find('\"')
                    .map(|i| i + 2)
                    .or_else(|| text.find('\n'))
                    .unwrap_or(text.len());
                job.append(&text[..end], 0.0, editor.format(TokenType::Str));
                text = &text[end..];
            } else if text.starts_with('\'') {
                let end = text[1..]
                    .find('\'')
                    .map(|i| i + 2)
                    .or_else(|| text.find('\n'))
                    .unwrap_or(text.len());
                job.append(&text[..end], 0.0, editor.format(TokenType::Str));
                text = &text[end..];
            } else if text.starts_with('`') {
                let end = text[1..]
                    .find('`')
                    .map(|i| i + 2)
                    .or_else(|| text.find('\n'))
                    .unwrap_or(text.len());
                job.append(&text[..end], 0.0, editor.format(TokenType::Str));
                text = &text[end..];
            }
            // Keyword | Type | Literal | Function
            else if text.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
                let end = text[1..]
                    .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .map_or_else(|| text.len(), |i| i + 1);
                let word = &text[..end];

                let tt = if editor.syntax.is_keyword(word) {
                    TokenType::Keyword
                } else if editor.syntax.is_type(word) {
                    TokenType::Type
                } else if editor.syntax.is_special(word) {
                    TokenType::Special
                } else if let Some('(') = text.chars().nth(end) {
                    TokenType::Function
                } else {
                    TokenType::Literal
                };

                job.append(word, 0.0, editor.format(tt));
                text = &text[end..];
            }
            // Whitespace
            else if text.starts_with(|c: char| c.is_ascii_whitespace()) {
                let end = text[1..]
                    .find(|c: char| !c.is_ascii_whitespace())
                    .map_or_else(|| text.len(), |i| i + 1);
                job.append(&text[..end], 0.0, editor.format(TokenType::Whitespace));
                text = &text[end..];
            }
            // Punctuation
            else {
                let mut it = text.char_indices();
                it.next();
                let end = it.next().map_or(text.len(), |(idx, _chr)| idx);
                job.append(&text[..end], 0.0, editor.format(TokenType::Punctuation));
                text = &text[end..];
            }
        }

        job
    }
}
