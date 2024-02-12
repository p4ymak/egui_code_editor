#![allow(dead_code)]
pub mod asm;
pub mod lua;
pub mod python;
pub mod rust;
pub mod shell;
pub mod sql;

use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

pub const SEPARATORS: [char; 1] = ['_'];
pub const QUOTES: [char; 3] = ['\'', '"', '`'];

type MultiLine = bool;
type Float = bool;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TokenType {
    Comment(MultiLine),
    Function,
    Keyword,
    Literal,
    Hyperlink,
    Numeric(Float),
    Punctuation(char),
    Special,
    Str(char),
    Type,
    Whitespace(char),
    #[default]
    Unknown,
}
impl std::fmt::Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = String::new();
        match &self {
            TokenType::Comment(multiline) => {
                name.push_str("Comment");
                {
                    if *multiline {
                        name.push_str(" MultiLine");
                    } else {
                        name.push_str(" SingleLine");
                    }
                }
            }
            TokenType::Function => name.push_str("Function"),
            TokenType::Keyword => name.push_str("Keyword"),
            TokenType::Literal => name.push_str("Literal"),
            TokenType::Hyperlink => name.push_str("Hyperlink"),
            TokenType::Numeric(float) => {
                name.push_str("Numeric");
                if *float {
                    name.push_str(" Float");
                } else {
                    name.push_str(" Integer");
                }
            }
            TokenType::Punctuation(_) => name.push_str("Punctuation"),
            TokenType::Special => name.push_str("Special"),
            TokenType::Str(quote) => {
                name.push_str("Str ");
                name.push(*quote);
            }
            TokenType::Type => name.push_str("Type"),
            TokenType::Whitespace(c) => {
                name.push_str("Whitespace");
                match c {
                    ' ' => name.push_str(" Space"),
                    '\t' => name.push_str(" Tab"),
                    '\n' => name.push_str(" New Line"),
                    _ => (),
                };
            }
            TokenType::Unknown => name.push_str("Unknown"),
        };
        write!(f, "{name}")
    }
}
impl From<char> for TokenType {
    fn from(c: char) -> Self {
        match c {
            c if c.is_whitespace() => TokenType::Whitespace(c),
            c if QUOTES.contains(&c) => TokenType::Str(c),
            c if c.is_numeric() => TokenType::Numeric(false),
            c if c.is_alphabetic() || SEPARATORS.contains(&c) => TokenType::Literal,
            c if c.is_ascii_punctuation() => TokenType::Punctuation(c),
            _ => TokenType::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Rules for highlighting.
pub struct Syntax {
    pub language: &'static str,
    pub case_sensitive: bool,
    pub comment: &'static str,
    pub comment_multiline: [&'static str; 2],
    pub hyperlinks: BTreeSet<&'static str>,
    pub keywords: BTreeSet<&'static str>,
    pub types: BTreeSet<&'static str>,
    pub special: BTreeSet<&'static str>,
}
impl Default for Syntax {
    fn default() -> Self {
        Syntax::rust()
    }
}
impl Hash for Syntax {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.language.hash(state);
    }
}
impl Syntax {
    pub fn new(language: &'static str) -> Self {
        Syntax {
            language,
            ..Default::default()
        }
    }
    pub fn with_case_sensitive(self, case_sensitive: bool) -> Self {
        Syntax {
            case_sensitive,
            ..self
        }
    }
    pub fn with_comment(self, comment: &'static str) -> Self {
        Syntax { comment, ..self }
    }
    pub fn with_comment_multiline(self, comment_multiline: [&'static str; 2]) -> Self {
        Syntax {
            comment_multiline,
            ..self
        }
    }
    pub fn with_hyperlinks<T: Into<BTreeSet<&'static str>>>(self, hyperlinks: T) -> Self {
        Syntax {
            hyperlinks: hyperlinks.into(),
            ..self
        }
    }
    pub fn with_keywords<T: Into<BTreeSet<&'static str>>>(self, keywords: T) -> Self {
        Syntax {
            keywords: keywords.into(),
            ..self
        }
    }
    pub fn with_types<T: Into<BTreeSet<&'static str>>>(self, types: T) -> Self {
        Syntax {
            types: types.into(),
            ..self
        }
    }
    pub fn with_special<T: Into<BTreeSet<&'static str>>>(self, special: T) -> Self {
        Syntax {
            special: special.into(),
            ..self
        }
    }

    pub fn language(&self) -> &str {
        self.language
    }
    pub fn comment(&self) -> &str {
        self.comment
    }
    pub fn is_hyperlink(&self, word: &str) -> bool {
        self.hyperlinks.contains(word.to_ascii_lowercase().as_str())
    }
    pub fn is_keyword(&self, word: &str) -> bool {
        if self.case_sensitive {
            self.keywords.contains(&word)
        } else {
            self.keywords.contains(word.to_ascii_uppercase().as_str())
        }
    }
    pub fn is_type(&self, word: &str) -> bool {
        if self.case_sensitive {
            self.types.contains(&word)
        } else {
            self.types.contains(word.to_ascii_uppercase().as_str())
        }
    }
    pub fn is_special(&self, word: &str) -> bool {
        if self.case_sensitive {
            self.special.contains(&word)
        } else {
            self.special.contains(word.to_ascii_uppercase().as_str())
        }
    }
}

impl Syntax {
    pub fn simple(comment: &'static str) -> Self {
        Syntax {
            language: "",
            case_sensitive: false,
            comment,
            comment_multiline: [comment; 2],
            hyperlinks: BTreeSet::new(),
            keywords: BTreeSet::new(),
            types: BTreeSet::new(),
            special: BTreeSet::new(),
        }
    }
}
