#[cfg(feature = "editor")]
use super::Editor;

use super::syntax::{Syntax, TokenType, QUOTES, SEPARATORS};
use std::mem;

#[derive(Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
/// Lexer and Token
pub struct Token {
    ty: TokenType,
    buffer: String,
}

impl Token {
    pub fn new<S: Into<String>>(ty: TokenType, buffer: S) -> Self {
        Token {
            ty,
            buffer: buffer.into(),
        }
    }
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
            c if syntax.comment == c.to_string().as_str() => TokenType::Comment(false),
            c if syntax.comment_multiline[0] == c.to_string().as_str() => TokenType::Comment(true),
            _ => TokenType::from(c),
        };
        token
    }

    fn drain(&mut self, ty: TokenType) -> Option<Self> {
        let mut token = None;
        if !self.buffer().is_empty() {
            token = Some(Token {
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

    #[cfg(feature = "egui")]
    /// Syntax highlighting
    pub fn highlight<T: Editor>(&mut self, editor: &T, text: &str) -> LayoutJob {
        *self = Token::default();
        let mut job = LayoutJob::default();
        for c in text.chars() {
            for token in self.automata(c, editor.syntax()) {
                editor.append(&mut job, &token);
            }
        }
        editor.append(&mut job, self);
        job
    }

    /// Lexer
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
        use TokenType as Ty;
        let mut tokens = vec![];
        match (self.ty, Ty::from(c)) {
            (Ty::Comment(false), Ty::Whitespace('\n')) => {
                self.buffer.push(c);
                let n = self.buffer.pop();
                tokens.extend(self.drain(Ty::Whitespace(c)));
                if let Some(n) = n {
                    tokens.extend(self.push_drain(n, self.ty));
                }
            }
            (Ty::Comment(false), _) => {
                self.buffer.push(c);
            }
            (Ty::Comment(true), _) => {
                self.buffer.push(c);
                if self.buffer.ends_with(syntax.comment_multiline[1]) {
                    tokens.extend(self.drain(Ty::Unknown));
                }
            }
            (Ty::Literal | Ty::Punctuation(_), Ty::Whitespace(_)) => {
                tokens.extend(self.drain(Ty::Whitespace(c)));
                tokens.extend(self.first(c, syntax));
            }
            (Ty::Hyperlink, Ty::Whitespace(_)) => {
                tokens.extend(self.drain(Ty::Whitespace(c)));
                tokens.extend(self.first(c, syntax));
            }
            (Ty::Hyperlink, _) => {
                self.buffer.push(c);
            }
            (Ty::Literal, _) => match c {
                c if c == '(' => {
                    self.ty = Ty::Function;
                    tokens.extend(self.drain(Ty::Punctuation(c)));
                    tokens.extend(self.push_drain(c, Ty::Unknown));
                }
                c if !c.is_alphanumeric() && !SEPARATORS.contains(&c) => {
                    tokens.extend(self.drain(self.ty));
                    self.buffer.push(c);
                    self.ty = if QUOTES.contains(&c) {
                        Ty::Str(c)
                    } else {
                        Ty::Punctuation(c)
                    };
                }
                _ => {
                    self.buffer.push(c);
                    self.ty = {
                        if self.buffer.starts_with(syntax.comment) {
                            Ty::Comment(false)
                        } else if self.buffer.starts_with(syntax.comment_multiline[0]) {
                            Ty::Comment(true)
                        } else if syntax.is_hyperlink(&self.buffer) {
                            Ty::Hyperlink
                        } else if syntax.is_keyword(&self.buffer) {
                            Ty::Keyword
                        } else if syntax.is_type(&self.buffer) {
                            Ty::Type
                        } else if syntax.is_special(&self.buffer) {
                            Ty::Special
                        } else {
                            Ty::Literal
                        }
                    };
                }
            },
            (Ty::Numeric(false), Ty::Punctuation('.')) => {
                self.buffer.push(c);
                self.ty = Ty::Numeric(true);
            }
            (Ty::Numeric(_), Ty::Numeric(_)) => {
                self.buffer.push(c);
            }
            (Ty::Numeric(_), Ty::Literal) => {
                tokens.extend(self.drain(self.ty));
                self.buffer.push(c);
            }
            (Ty::Numeric(_), _) | (Ty::Punctuation(_), Ty::Literal | Ty::Numeric(_)) => {
                tokens.extend(self.drain(self.ty));
                tokens.extend(self.first(c, syntax));
            }
            (Ty::Punctuation(_), Ty::Str(_)) => {
                tokens.extend(self.drain_push(c, Ty::Str(c)));
            }
            (Ty::Punctuation(_), _) => {
                if !(syntax.comment.starts_with(&self.buffer)
                    || syntax.comment_multiline[0].starts_with(&self.buffer))
                {
                    tokens.extend(self.drain(self.ty));
                    tokens.extend(self.first(c, syntax));
                } else {
                    self.buffer.push(c);
                    if self.buffer.starts_with(syntax.comment) {
                        self.ty = Ty::Comment(false);
                    } else if self.buffer.starts_with(syntax.comment_multiline[0]) {
                        self.ty = Ty::Comment(true);
                    } else if let Some(c) = self.buffer.pop() {
                        tokens.extend(self.drain(Ty::Punctuation(c)));
                        tokens.extend(self.first(c, syntax));
                    }
                }
            }
            (Ty::Str(q), _) => {
                let control = self.buffer.ends_with('\\');
                self.buffer.push(c);
                if c == q && !control {
                    tokens.extend(self.drain(Ty::Unknown));
                }
            }
            (Ty::Whitespace(_) | Ty::Unknown, _) => {
                tokens.extend(self.first(c, syntax));
            }
            // Keyword, Type, Special
            (_reserved, Ty::Literal | Ty::Numeric(_)) => {
                self.buffer.push(c);
                self.ty = if syntax.is_keyword(&self.buffer) {
                    Ty::Keyword
                } else if syntax.is_type(&self.buffer) {
                    Ty::Type
                } else if syntax.is_special(&self.buffer) {
                    Ty::Special
                } else {
                    Ty::Literal
                };
            }
            (reserved, _) => {
                self.ty = reserved;
                tokens.extend(self.drain(self.ty));
                tokens.extend(self.first(c, syntax));
            }
        }
        tokens
    }
}

#[cfg(feature = "egui")]
use egui::text::LayoutJob;

#[cfg(feature = "egui")]
impl<T: Editor> egui::util::cache::ComputerMut<(&T, &str), LayoutJob> for Token {
    fn compute(&mut self, (cache, text): (&T, &str)) -> LayoutJob {
        self.highlight(cache, text)
    }
}

#[cfg(feature = "egui")]
pub type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Token>;

#[cfg(feature = "egui")]
pub fn highlight<T: Editor>(ctx: &egui::Context, cache: &T, text: &str) -> LayoutJob {
    ctx.memory_mut(|mem| mem.caches.cache::<HighlightCache>().get((cache, text)))
}
