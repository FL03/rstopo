/*
    Appellation: simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplex
//!
//! Simplexes are polytopes that are equivalent to the convex hull of their vertices. This module
//! provides the necessary tools to work with simplexes in n-dimensional space.
//! 
pub use self::simplex::*;

pub(crate) mod simplex;

pub(crate) mod prelude {
    pub use super::simplex::Simplex;
}
