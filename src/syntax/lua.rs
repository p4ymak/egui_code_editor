use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
    pub fn lua() -> Syntax {
        Syntax {
            language: "Lua",
            case_sensitive: true,
            comment: "--",
            comment_multiline: ["--[[", "]]"],
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from([
                "and", "break", "do", "else", "elseif", "end", "for", "function", "if", "in",
                "local", "not", "or", "repeat", "return", "then", "until", "while",
            ]),
            types: BTreeSet::from([
                "boolean", "number", "string", "function", "userdata", "thread", "table",
            ]),
            special: BTreeSet::from(["false", "nil", "true"]),
        }
    }
}
