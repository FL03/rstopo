/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod linalg;
pub mod shape;

pub(crate) mod prelude {
    pub use super::linalg::*;
    pub use super::shape::Shape;
}
