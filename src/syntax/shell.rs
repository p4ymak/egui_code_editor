use super::Syntax;
use std::collections::HashSet;

impl Syntax {
    #[must_use]
    pub fn shell() -> Self {
        Syntax {
            language: "Shell",
            case_sensitive: false,
            comment: "#",
            keywords: HashSet::from([
                "echo", "read", "set", "unset", "readonly", "shift", "export", "if", "fi", "else",
                "while", "do", "done", "for", "until", "case", "esac", "break", "continue", "exit",
                "return", "trap", "wait", "eval", "exec", "ulimit", "umask",
            ]),
            types: HashSet::from([
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
            special: HashSet::from([
                "alias", "bg", "cd", "command", "false", "fc", "fg", "getopts", "jobs", "kill",
                "newgrp", "pwd", "read", "true", "umask", "unalias", "wait",
            ]),
        }
    }
}
