/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstopo
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rstopo"]

#[doc(inline)]
pub use rstopo_core::*;
#[cfg(feature = "simplex")]
#[doc(inline)]
pub use rstopo_simplex as simplex;

pub mod prelude {
    pub use rstopo_core::prelude::*;
    #[cfg(feature = "simplex")]
    pub use rstopo_simplex::prelude::*;
}
