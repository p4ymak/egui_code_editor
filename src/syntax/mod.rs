#![allow(dead_code)]
pub mod rust;
pub mod shell;
pub mod sql;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TokenType {
    Comment,
    Function,
    Keyword,
    Literal,
    Numeric,
    Punctuation,
    Special,
    Str,
    Type,
    Whitespace,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Syntax {
    pub language: &'static str,
    pub case_sensitive: bool,
    pub comment: &'static str,
    pub comment_multiline: [&'static str; 2],
    pub keywords: HashSet<&'static str>,
    pub types: HashSet<&'static str>,
    pub special: HashSet<&'static str>,
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
            keywords: HashSet::new(),
            types: HashSet::new(),
            special: HashSet::new(),
        }
    }
}
