/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # ops
//!
//! The module implements any overloadable operations that may be performed on a shape / topological space.
#[doc(inline)]
pub use self::prelude::*;

pub mod distance;
pub mod props;

pub(crate) mod prelude {
    pub use super::distance::Distance;
    pub use super::props::*;
}
