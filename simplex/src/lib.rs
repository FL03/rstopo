/*
    Appellation: rstopo-simplex <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstopo
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use simplex::Simplex;

pub mod simplex;

pub mod prelude {
    pub use super::simplex::prelude::*;
}
