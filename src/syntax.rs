#![allow(dead_code)]

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
    Str,
    Type,
    Whitespace,
}

#[derive(Clone)]
pub struct Syntax {
    language: &'static str,
    case_sensitive: bool,
    comment: &'static str,
    keywords: HashSet<&'static str>,
    types: HashSet<&'static str>,
}
impl Hash for Syntax {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.language.hash(state);
    }
}
impl Syntax {
    pub fn comment(&self) -> &str {
        self.comment
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
}

impl Syntax {
    pub fn sql() -> Self {
        Syntax {
            language: "SQL",
            case_sensitive: false,
            comment: "--",
            keywords: HashSet::from([
                "ADD",
                "ALL",
                "ALTER",
                "AND",
                "ANY",
                "AS",
                "ASC",
                "BACKUP",
                "BETWEEN",
                "CASE",
                "CHECK",
                "COLUMN",
                "CONSTRAINT",
                "CREATE",
                "DATABASE",
                "INDEX",
                "OR",
                "REPLACE",
                "VIEW",
                "PROCEDURE",
                "UNIQUE",
                "DEFAULT",
                "DELETE",
                "DESC",
                "DISTINCT",
                "DROP",
                "PRIMARY",
                "FOREIGN",
                "EXEC",
                "EXISTS",
                "FROM",
                "FULL",
                "OUTER",
                "JOIN",
                "GROUP",
                "BY",
                "HAVING",
                "IN",
                "INNER",
                "INSERT",
                "INTO",
                "SELECT",
                "IS",
                "NOT",
                "NULL",
                "LEFT",
                "LIKE",
                "LIMIT",
                "ORDER",
                "A",
                "RIGHT",
                "ROWNUM",
                "TOP",
                "SET",
                "TRUNCATE",
                "UPDATE",
                "VALUES",
                "WHERE",
                "WITH",
            ]),
            types: HashSet::from(["TABLE", "KEY", "UNION", "TEXT", "VARCHAR", "INT4"]),
        }
    }
}
