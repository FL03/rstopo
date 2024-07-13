/*
    Appellation: err <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{ErrorKind, ErrorLevel};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error<K = String> {
    pub(crate) kind: Option<K>,
    pub(crate) level: ErrorLevel,
    pub(crate) message: String,
}

impl<K> Error<K>
where
    K: ErrorKind,
{
    pub fn new(kind: K, message: impl ToString) -> Self {
        Self {
            kind: Some(kind),
            level: ErrorLevel::default(),
            message: message.to_string(),
        }
    }

    pub fn unknown(message: impl ToString) -> Self {
        Self {
            kind: None,
            level: ErrorLevel::default(),
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> Option<&K> {
        self.kind.as_ref()
    }

    pub fn level(&self) -> ErrorLevel {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn message_mut(&mut self) -> &mut String {
        &mut self.message
    }

    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string();
    }

    pub fn with_kind<K2>(self, kind: K2) -> Error<K2> {
        Error {
            kind: Some(kind),
            level: self.level,
            message: self.message,
        }
    }

    pub fn with_message(self, message: impl ToString) -> Self {
        Self {
            kind: self.kind,
            level: self.level,
            message: message.to_string(),
        }
    }
}

#[cfg(feature = "std")]
impl<K> std::error::Error for Error<K> where K: ErrorKind {}

impl<K> core::fmt::Display for Error<K>
where
    K: ErrorKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(ref kind) = self.kind {
            f.write_str(&format!("{}: ", kind))?;
        }
        write!(f, "{}", self.message)
    }
}

impl<K> From<K> for Error<K>
where
    K: ErrorKind,
{
    fn from(kind: K) -> Self {
        Self::new(kind, "")
    }
}
