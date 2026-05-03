use std::fmt;
use std::io;

#[derive(Debug)]
pub enum BdfError {
    Io(io::Error),

    Syntax {
        line: Option<usize>,
        context: String,
        message: String,
    },
    
    Parse {
        line: Option<usize>,
        keyword: String,
        value: String,
        reason: String,
    },

    Integrity {
        line: Option<usize>,
        context: String,
        message: String,
    },
}

impl fmt::Display for BdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BdfError::Io(err) => write!(f, "IO error: {}", err),
            BdfError::Syntax { line, context, message } => {
                let line_str = line.map(|l| format!(" at line {}", l)).unwrap_or_default();
                if context.is_empty() {
                    write!(f, "Syntax error{}: {}", line_str, message)
                } else {
                    write!(f, "Syntax error{} in {}: {}", line_str, context, message)
                }
            },
            BdfError::Parse { line, keyword, value, reason } => {
                let line_str = line.map(|l| format!(" at line {}", l)).unwrap_or_default();
                if keyword.is_empty() {
                    write!(f, "Parse error{}: Failed to parse '{}'. Reason: {}", line_str, value, reason)
                } else {
                    write!(f, "Parse error{}: Failed to parse '{}' for keyword '{}'. Reason: {}", line_str, value, keyword, reason)
                }
            },
            BdfError::Integrity { line, context, message } => {
                let line_str = line.map(|l| format!(" at line {}", l)).unwrap_or_default();
                if context.is_empty() {
                    write!(f, "Integrity error{}: {}", line_str, message)
                } else {
                    write!(f, "Integrity error{} in {}: {}", line_str, context, message)
                }
            },
        }
    }
}

impl std::error::Error for BdfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BdfError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for BdfError {
    fn from(error: io::Error) -> Self {
        BdfError::Io(error)
    }
}

impl BdfError {
    pub fn syntax(line: Option<usize>, message: impl Into<String>) -> Self {
        BdfError::Syntax {
            line,
            context: String::new(),
            message: message.into(),
        }
    }

    pub fn syntax_in(line: Option<usize>, context: impl Into<String>, message: impl Into<String>) -> Self {
        BdfError::Syntax {
            line,
            context: context.into(),
            message: message.into(),
        }
    }

    pub fn parse(line: Option<usize>, keyword: impl Into<String>, value: impl Into<String>, reason: impl Into<String>) -> Self {
        BdfError::Parse {
            line,
            keyword: keyword.into(),
            value: value.into(),
            reason: reason.into(),
        }
    }

    pub fn integrity(line: Option<usize>, message: impl Into<String>) -> Self {
        BdfError::Integrity {
            line,
            context: String::new(),
            message: message.into(),
        }
    }

    pub fn integrity_in(line: Option<usize>, context: impl Into<String>, message: impl Into<String>) -> Self {
        BdfError::Integrity {
            line,
            context: context.into(),
            message: message.into(),
        }
    }
}