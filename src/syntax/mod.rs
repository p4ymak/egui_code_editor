#![allow(dead_code)]
pub mod asm;
pub mod lua;
pub mod rust;
pub mod shell;
pub mod sql;

use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

type Multiline = bool;

#[derive(Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TokenType {
    Comment(Multiline),
    Function,
    Keyword,
    Literal,
    Numeric,
    Punctuation,
    Special,
    Str(char),
    Type,
    #[default]
    Whitespace,
    NewLine,
}
impl std::fmt::Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = String::new();
        match &self {
            TokenType::Comment(multiline) => {
                name.push_str("Comment");
                {
                    if *multiline {
                        name.push_str(" Multiline");
                    } else {
                        name.push_str(" Singleline");
                    }
                }
            }
            TokenType::Function => name.push_str("Function"),
            TokenType::Keyword => name.push_str("Keyword"),
            TokenType::Literal => name.push_str("Literal"),
            TokenType::Numeric => name.push_str("Numeric"),
            TokenType::Punctuation => name.push_str("Punctuation"),
            TokenType::Special => name.push_str("Special"),
            TokenType::Str(quote) => {
                name.push_str("Str ");
                name.push(*quote);
            }

            TokenType::Type => name.push_str("Type"),
            TokenType::Whitespace => name.push_str("Whitespace"),
            TokenType::NewLine => name.push_str("NewLine"),
        };
        write!(f, "{name}")
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Rules for highlighting.
pub struct Syntax {
    pub language: &'static str,
    pub case_sensitive: bool,
    pub comment: &'static str,
    pub comment_multiline: [&'static str; 2],
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
    #[must_use]
    pub fn language(&self) -> &str {
        self.language
    }
    #[must_use]
    pub fn comment(&self) -> &str {
        self.comment
    }
    #[must_use]
    pub fn is_keyword(&self, word: &str) -> bool {
        if self.case_sensitive {
            self.keywords.contains(&word)
        } else {
            self.keywords.contains(word.to_ascii_uppercase().as_str())
        }
    }
    #[must_use]
    pub fn is_type(&self, word: &str) -> bool {
        if self.case_sensitive {
            self.types.contains(&word)
        } else {
            self.types.contains(word.to_ascii_uppercase().as_str())
        }
    }
    #[must_use]
    pub fn is_special(&self, word: &str) -> bool {
        if self.case_sensitive {
            self.special.contains(&word)
        } else {
            self.special.contains(word.to_ascii_uppercase().as_str())
        }
    }
}

impl Syntax {
    #[must_use]
    pub fn simple(comment: &'static str) -> Self {
        Syntax {
            language: "",
            case_sensitive: false,
            comment,
            comment_multiline: [comment; 2],
            keywords: BTreeSet::new(),
            types: BTreeSet::new(),
            special: BTreeSet::new(),
        }
    }
}
