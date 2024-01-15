use std::path::{Path, PathBuf};

use thiserror::Error;

use super::pos::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Eof,
    NullCharacter,
    UnescapedSpecialCharacter(char),
    UnterminatedSingleQuotedString,
    UnterminatedDoubleQuotedString,
    UnsupportedShellParameter(String),
    UnterminatedExpansion,
    UnsupportedCommandExpansion,
    UnsupportedCommandOrArithmeticExpansion,
    InvalidCharacter(char),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "Unexpected end of input"),
            Self::NullCharacter => write!(f, "Unexpected <NUL> character"),
            Self::UnescapedSpecialCharacter(ch) => {
                write!(f, "Unescaped special shell character '{ch}'")
            }
            Self::UnterminatedSingleQuotedString => {
                write!(f, "Unterminated single-quoted string")
            }
            Self::UnterminatedDoubleQuotedString => {
                write!(f, "Unterminated double-quoted string")
            }
            Self::UnsupportedShellParameter(p) => {
                write!(f, "Unsupported special shell parameter: {p}")
            }
            Self::UnterminatedExpansion => write!(f, "Unterminated expansion"),
            Self::UnsupportedCommandExpansion => write!(f, "Unsupported command expansion"),
            Self::UnsupportedCommandOrArithmeticExpansion => {
                write!(f, "Unsupported command or arithmetic expansion")
            }
            Self::InvalidCharacter(ch) => write!(f, "Invalid character '{ch}'"),
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub struct SyntaxError {
    kind: ErrorKind,
    position: Position,
    filename: Option<PathBuf>,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind().fmt(f)?;
        if let Some(file) = self.file() {
            write!(f, " in {}", file.display())?;
        }
        write!(f, " on line {}, column {}", self.line(), self.column())
    }
}

impl SyntaxError {
    pub fn new(kind: ErrorKind, position: Position, filename: Option<PathBuf>) -> Self {
        Self {
            kind,
            position,
            filename,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn line(&self) -> usize {
        self.position.line
    }

    pub fn column(&self) -> usize {
        self.position.column
    }

    pub fn file(&self) -> Option<&Path> {
        self.filename.as_deref()
    }
}
