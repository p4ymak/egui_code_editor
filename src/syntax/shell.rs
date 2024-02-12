use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
    pub fn shell() -> Self {
        Syntax {
            language: "Shell",
            case_sensitive: true,
            comment: "#",
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from([
                "echo", "read", "set", "unset", "readonly", "shift", "export", "if", "fi", "else",
                "while", "do", "done", "for", "until", "case", "esac", "break", "continue", "exit",
                "return", "trap", "wait", "eval", "exec", "ulimit", "umask",
            ]),
            comment_multiline: [": '", "'"],
            types: BTreeSet::from([
                "ENV",
                "HOME",
                "IFS",
                "LANG",
                "LC_ALL",
                "LC_COLLATE",
                "LC_CTYPE",
                "LC_MESSAGES",
                "LINENO",
                "NLSPATH",
                "PATH",
                "PPID",
                "PS1",
                "PS2",
                "PS4",
                "PWD",
            ]),
            special: BTreeSet::from([
                "alias", "bg", "cd", "command", "false", "fc", "fg", "getopts", "jobs", "kill",
                "newgrp", "pwd", "read", "true", "umask", "unalias", "wait",
            ]),
        }
    }
}
