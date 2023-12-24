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
    fn first(&mut self, c: char, editor: &CodeEditor, job: &mut LayoutJob) {
        self.buffer.push(c);
        self.ty = match c {
            c if c.is_whitespace() => {
                self.drain(editor, job, self.ty);
                TokenType::Whitespace
            }
            c if c.is_alphabetic() || SEPARATORS.contains(&c) => TokenType::Literal,
            c if c.is_numeric() => TokenType::Numeric,
            c if editor.syntax.comment == c.to_string().as_str() => TokenType::Comment(false),
            c if editor.syntax.comment_multiline[0] == c.to_string().as_str() => {
                TokenType::Comment(true)
            }
            c if QUOTES.contains(&c) => TokenType::Str(c),
            _ => TokenType::Punctuation,
        };
    }

    fn drain(&mut self, editor: &CodeEditor, job: &mut LayoutJob, ty: TokenType) {
        editor.append(job, &self.buffer, self.ty);
        self.buffer.clear();
        self.ty = ty;
    }

    fn push_drain(&mut self, c: char, editor: &CodeEditor, job: &mut LayoutJob, ty: TokenType) {
        self.buffer.push(c);
        self.drain(editor, job, ty);
    }

    fn drain_push(&mut self, c: char, editor: &CodeEditor, job: &mut LayoutJob, ty: TokenType) {
        self.drain(editor, job, self.ty);
        self.buffer.push(c);
        self.ty = ty;
    }

    pub fn highlight(&mut self, editor: &CodeEditor, text: &str) -> LayoutJob {
        *self = Highlighter::default();
        let mut job = LayoutJob::default();
        for c in text.chars() {
            self.automata(c, editor, &mut job);
        }
        self.drain(editor, &mut job, TokenType::Whitespace);
        job
    }

    fn automata(&mut self, c: char, editor: &CodeEditor, job: &mut LayoutJob) {
        match self.ty {
            TokenType::Comment(multiline) => {
                self.buffer.push(c);
                if !multiline {
                    if c == '\n' {
                        self.drain(editor, job, TokenType::Whitespace);
                    }
                } else if self.buffer.ends_with(editor.syntax.comment_multiline[1]) {
                    self.drain(editor, job, TokenType::Whitespace);
                }
            }
            TokenType::Literal => match c {
                c if c.is_whitespace() => {
                    self.push_drain(c, editor, job, TokenType::Whitespace);
                }
                c if c == '(' => {
                    self.ty = TokenType::Function;
                    self.drain(editor, job, TokenType::Punctuation);
                    self.push_drain(c, editor, job, TokenType::Whitespace);
                }
                c if !c.is_alphanumeric() && !SEPARATORS.contains(&c) => {
                    self.drain(editor, job, self.ty);
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
                        if self.buffer.starts_with(editor.syntax.comment) {
                            TokenType::Comment(false)
                        } else if self.buffer.starts_with(editor.syntax.comment_multiline[0]) {
                            TokenType::Comment(true)
                        } else if editor.syntax.is_keyword(&self.buffer) {
                            TokenType::Keyword
                        } else if editor.syntax.is_type(&self.buffer) {
                            TokenType::Type
                        } else if editor.syntax.is_special(&self.buffer) {
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
                    self.drain(editor, job, self.ty);
                    match c {
                        c if c.is_alphabetic() || c == '_' => {
                            self.buffer.push(c);
                        }
                        _ => {
                            self.first(c, editor, job);
                        }
                    }
                }
            }
            TokenType::Punctuation => match c {
                c if c.is_whitespace() => {
                    self.push_drain(c, editor, job, TokenType::Whitespace);
                }
                c if c.is_alphabetic() || SEPARATORS.contains(&c) => {
                    self.drain_push(c, editor, job, TokenType::Literal);
                }
                c if c.is_numeric() => {
                    self.drain_push(c, editor, job, TokenType::Numeric);
                }
                c if QUOTES.contains(&c) => {
                    self.drain_push(c, editor, job, TokenType::Str(c));
                }
                _ => {
                    if !(editor.syntax.comment.starts_with(&self.buffer)
                        || editor.syntax.comment_multiline[0].starts_with(&self.buffer))
                    {
                        self.drain(editor, job, self.ty);
                        self.first(c, editor, job);
                    } else {
                        self.buffer.push(c);
                        if self.buffer.starts_with(editor.syntax.comment) {
                            self.ty = TokenType::Comment(false);
                        } else if self.buffer.starts_with(editor.syntax.comment_multiline[0]) {
                            self.ty = TokenType::Comment(true);
                        } else if let Some(c) = self.buffer.pop() {
                            self.drain(editor, job, TokenType::Punctuation);
                            self.first(c, editor, job);
                        }
                    }
                }
            },
            TokenType::Str(q) => {
                let control = self.buffer.ends_with('\\');
                self.buffer.push(c);
                if c == q && !control {
                    self.drain(editor, job, TokenType::Whitespace);
                }
            }
            TokenType::Whitespace => {
                self.first(c, editor, job);
            }
            // Keyword, Type, Special
            reserved => {
                if !(c.is_alphanumeric() || SEPARATORS.contains(&c)) {
                    self.ty = reserved;
                    self.drain(editor, job, self.ty);
                    self.first(c, editor, job);
                } else {
                    self.buffer.push(c);
                    self.ty = TokenType::Literal;
                }
            }
        }
    }
}

impl CodeEditor {
    fn append(&self, job: &mut LayoutJob, text: &str, ty: TokenType) {
        job.append(text, 0.0, self.format(ty));
    }
}
