/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{err::Error, level::ErrorLevel};

pub(crate) mod err;

pub mod level;

pub(crate) mod prelude {
    pub use super::err::Error;
    pub use super::level::ErrorLevel;
    pub use super::{ErrorKind, IntoError};
}

pub trait IntoError<K> {
    fn into_error(self) -> Error<K>;
}

impl<E> IntoError<E> for E
where
    E: ErrorKind,
{
    fn into_error(self) -> Error<E> {
        Error::new(self, "")
    }
}
pub trait ErrorKind: core::fmt::Debug + core::fmt::Display + Send + Sync {}

macro_rules! impl_error_kind {
    ($($t:ty),*) => {
        $(
            impl ErrorKind for $t {}
        )*
    };
}

impl_error_kind!(String, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl<'a> ErrorKind for &'a str {}
