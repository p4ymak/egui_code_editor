use super::syntax::TokenType;
use super::CodeEditor;
use egui::text::LayoutJob;

const SEPARATORS: [char; 2] = ['_', '-'];
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
    fn first(&mut self, c: char) {
        self.buffer.push(c);
        self.ty = match c {
            c if c.is_alphabetic() || SEPARATORS.contains(&c) => TokenType::Literal,
            c if c.is_numeric() => TokenType::Numeric,
            c if QUOTES.contains(&c) => TokenType::Str(c),
            _ => TokenType::Punctuation,
        };
    }
    fn drain(&mut self, editor: &CodeEditor, job: &mut LayoutJob) {
        editor.append(job, &self.buffer, self.ty);
        *self = Highlighter::default();
    }
    pub fn highlight(&mut self, editor: &CodeEditor, text: &str) -> LayoutJob {
        *self = Highlighter::default();
        let mut job = LayoutJob::default();
        for c in text.chars() {
            self.automata(c, editor, &mut job);
        }
        self.drain(editor, &mut job);
        job
    }
    fn automata(&mut self, c: char, editor: &CodeEditor, job: &mut LayoutJob) {
        match self.ty {
            TokenType::Comment => {
                self.buffer.push(c);
                if self.buffer.starts_with(editor.syntax.comment) {
                    if c == '\n' {
                        self.drain(editor, job);
                        self.ty = TokenType::Whitespace;
                    }
                } else if self.buffer.ends_with(editor.syntax.comment_multiline[1]) {
                    self.drain(editor, job);
                    self.ty = TokenType::Whitespace;
                }
            }
            TokenType::Literal => match c {
                c if c.is_whitespace() => {
                    self.buffer.push(c);
                    self.drain(editor, job);
                    self.ty = TokenType::Whitespace;
                }
                c if c == '(' => {
                    self.ty = TokenType::Function;
                    self.drain(editor, job);
                    self.ty = TokenType::Punctuation;
                    self.buffer.push(c);
                    self.drain(editor, job);
                    self.ty = TokenType::Whitespace;
                }
                c if !c.is_alphanumeric() && !SEPARATORS.contains(&c) => {
                    self.drain(editor, job);
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
                        if self.buffer.starts_with(editor.syntax.comment)
                            || self.buffer.starts_with(editor.syntax.comment_multiline[0])
                        {
                            TokenType::Comment
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
                    self.drain(editor, job);
                    match c {
                        c if c.is_alphabetic() => {
                            self.buffer.push(c);
                        }
                        _ => {
                            self.first(c);
                        }
                    }
                }
            }
            TokenType::Punctuation => match c {
                c if c.is_alphabetic() || SEPARATORS.contains(&c) => {
                    self.drain(editor, job);
                    self.buffer.push(c);
                    self.ty = TokenType::Literal;
                }
                c if c.is_numeric() => {
                    self.drain(editor, job);
                    self.buffer.push(c);
                    self.ty = TokenType::Numeric;
                }
                c if QUOTES.contains(&c) => {
                    self.drain(editor, job);
                    self.buffer.push(c);
                    self.ty = TokenType::Str(c);
                }
                _ => {
                    self.buffer.push(c);

                    if editor.syntax.comment.starts_with(&self.buffer)
                        || editor.syntax.comment_multiline[0].starts_with(&self.buffer)
                    {
                        if self.buffer == editor.syntax.comment
                            || self.buffer == editor.syntax.comment_multiline[0]
                        {
                            self.ty = TokenType::Comment;
                        }
                    } else {
                        self.drain(editor, job);
                    }
                }
            },
            TokenType::Str(q) => {
                self.buffer.push(c);
                if c == q {
                    self.drain(editor, job);
                    self.ty = TokenType::Whitespace;
                }
            }
            TokenType::Whitespace => {
                self.first(c);
            }
            // Keyword, Type, Special
            reserved => {
                if !(c.is_alphanumeric() || SEPARATORS.contains(&c)) {
                    self.ty = reserved;
                    self.drain(editor, job);
                    self.first(c);
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
