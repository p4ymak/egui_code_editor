use std::mem;

use crate::Syntax;

use super::syntax::TokenType;
use super::CodeEditor;
use egui::text::LayoutJob;

const SEPARATORS: [char; 1] = ['_'];
const QUOTES: [char; 3] = ['\'', '"', '`'];

pub type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Highlighter>;
pub fn highlight(ctx: &egui::Context, cache: &CodeEditor, text: &str) -> LayoutJob {
    ctx.memory_mut(|mem| mem.caches.cache::<HighlightCache>().get((cache, text)))
}

#[derive(Default)]
pub struct Highlighter {
    ty: TokenType,
    buffer: String,
}

impl egui::util::cache::ComputerMut<(&CodeEditor, &str), LayoutJob> for Highlighter {
    fn compute(&mut self, (cache, text): (&CodeEditor, &str)) -> LayoutJob {
        self.highlight(cache, text)
    }
}
impl Highlighter {
    pub fn ty(&self) -> TokenType {
        self.ty
    }
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    fn first(&mut self, c: char, syntax: &Syntax) -> Option<Self> {
        self.buffer.push(c);
        let mut token = None;
        self.ty = match c {
            c if c.is_whitespace() => {
                self.ty = TokenType::Whitespace(c);
                token = self.drain(self.ty);
                TokenType::Whitespace(c)
            }
            c if syntax.is_keyword(c.to_string().as_str()) => TokenType::Keyword,
            c if syntax.is_type(c.to_string().as_str()) => TokenType::Type,
            c if syntax.is_special(c.to_string().as_str()) => TokenType::Special,
            c if c.is_alphabetic() || SEPARATORS.contains(&c) => TokenType::Literal,
            c if c.is_numeric() => TokenType::Numeric,
            c if syntax.comment == c.to_string().as_str() => TokenType::Comment(false),
            c if syntax.comment_multiline[0] == c.to_string().as_str() => TokenType::Comment(true),
            c if QUOTES.contains(&c) => TokenType::Str(c),
            _ => TokenType::Punctuation,
        };
        token
    }

    fn drain(&mut self, ty: TokenType) -> Option<Self> {
        let mut token = None;
        if !self.buffer().is_empty() {
            token = Some(Highlighter {
                buffer: mem::take(&mut self.buffer),
                ty: self.ty,
            });
        }
        self.ty = ty;
        token
    }

    fn push_drain(&mut self, c: char, ty: TokenType) -> Option<Self> {
        self.buffer.push(c);
        self.drain(ty)
    }

    fn drain_push(&mut self, c: char, ty: TokenType) -> Option<Self> {
        let token = self.drain(self.ty);
        self.buffer.push(c);
        self.ty = ty;
        token
    }

    pub fn highlight(&mut self, editor: &CodeEditor, text: &str) -> LayoutJob {
        *self = Highlighter::default();
        let mut job = LayoutJob::default();
        for c in text.chars() {
            for token in self.automata(c, &editor.syntax) {
                editor.append(&mut job, &token);
            }
        }
        editor.append(&mut job, self);
        job
    }

    pub fn tokens(&mut self, syntax: &Syntax, text: &str) -> Vec<Self> {
        let mut tokens: Vec<Self> = text
            .chars()
            .flat_map(|c| self.automata(c, syntax))
            .collect();

        if !self.buffer.is_empty() {
            tokens.push(mem::take(self));
        }
        tokens
    }

    fn automata(&mut self, c: char, syntax: &Syntax) -> Vec<Self> {
        let mut tokens = vec![];
        match self.ty {
            TokenType::Comment(multiline) => {
                self.buffer.push(c);
                if !multiline {
                    if c == '\n' {
                        let n = self.buffer.pop();
                        tokens.extend(self.drain(TokenType::Whitespace(c)));
                        if let Some(n) = n {
                            tokens.extend(self.push_drain(n, self.ty));
                        }
                    }
                } else if self.buffer.ends_with(syntax.comment_multiline[1]) {
                    tokens.extend(self.drain(TokenType::Whitespace(c)));
                }
            }
            TokenType::Literal => match c {
                c if c.is_whitespace() => {
                    tokens.extend(self.drain(TokenType::Whitespace(c)));
                    tokens.extend(self.first(c, syntax));
                }

                c if c == '(' => {
                    self.ty = TokenType::Function;
                    tokens.extend(self.drain(TokenType::Punctuation));
                    tokens.extend(self.push_drain(c, TokenType::Unknown));
                }
                c if !c.is_alphanumeric() && !SEPARATORS.contains(&c) => {
                    tokens.extend(self.drain(self.ty));
                    self.buffer.push(c);
                    self.ty = if QUOTES.contains(&c) {
                        TokenType::Str(c)
                    } else {
                        TokenType::Punctuation
                    };
                }
                _ => {
                    self.buffer.push(c);
                    self.ty = {
                        if self.buffer.starts_with(syntax.comment) {
                            TokenType::Comment(false)
                        } else if self.buffer.starts_with(syntax.comment_multiline[0]) {
                            TokenType::Comment(true)
                        } else if syntax.is_keyword(&self.buffer) {
                            TokenType::Keyword
                        } else if syntax.is_type(&self.buffer) {
                            TokenType::Type
                        } else if syntax.is_special(&self.buffer) {
                            TokenType::Special
                        } else {
                            TokenType::Literal
                        }
                    };
                }
            },
            TokenType::Numeric => {
                if c.is_numeric() {
                    self.buffer.push(c);
                } else {
                    tokens.extend(self.drain(self.ty));
                    match c {
                        c if c.is_alphabetic() || c == '_' => {
                            self.buffer.push(c);
                        }
                        _ => {
                            tokens.extend(self.first(c, syntax));
                        }
                    }
                }
            }
            TokenType::Punctuation => match c {
                c if c.is_whitespace() => {
                    tokens.extend(self.drain(TokenType::Whitespace(c)));
                    tokens.extend(self.first(c, syntax));
                }
                c if c.is_alphanumeric() || SEPARATORS.contains(&c) => {
                    tokens.extend(self.drain(self.ty));
                    tokens.extend(self.first(c, syntax));
                }
                c if QUOTES.contains(&c) => {
                    tokens.extend(self.drain_push(c, TokenType::Str(c)));
                }
                _ => {
                    if !(syntax.comment.starts_with(&self.buffer)
                        || syntax.comment_multiline[0].starts_with(&self.buffer))
                    {
                        tokens.extend(self.drain(self.ty));
                        tokens.extend(self.first(c, syntax));
                    } else {
                        self.buffer.push(c);
                        if self.buffer.starts_with(syntax.comment) {
                            self.ty = TokenType::Comment(false);
                        } else if self.buffer.starts_with(syntax.comment_multiline[0]) {
                            self.ty = TokenType::Comment(true);
                        } else if let Some(c) = self.buffer.pop() {
                            tokens.extend(self.drain(TokenType::Punctuation));
                            tokens.extend(self.first(c, syntax));
                        }
                    }
                }
            },
            TokenType::Str(q) => {
                let control = self.buffer.ends_with('\\');
                self.buffer.push(c);
                if c == q && !control {
                    tokens.extend(self.drain(TokenType::Unknown));
                }
            }
            TokenType::Whitespace(_) | TokenType::Unknown => {
                tokens.extend(self.first(c, syntax));
            }
            // Keyword, Type, Special
            reserved => {
                if !(c.is_alphanumeric() || SEPARATORS.contains(&c)) {
                    self.ty = reserved;
                    tokens.extend(self.drain(self.ty));
                    tokens.extend(self.first(c, syntax));
                } else {
                    self.buffer.push(c);
                    self.ty = if syntax.is_keyword(&self.buffer) {
                        TokenType::Keyword
                    } else if syntax.is_type(&self.buffer) {
                        TokenType::Type
                    } else if syntax.is_special(&self.buffer) {
                        TokenType::Special
                    } else {
                        TokenType::Literal
                    };
                }
            }
        }
        tokens
    }
}

impl CodeEditor {
    fn append(&self, job: &mut LayoutJob, token: &Highlighter) {
        job.append(token.buffer(), 0.0, self.format(token.ty()));
    }
}
