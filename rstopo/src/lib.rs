/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstopo
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rstopo"]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod simplex;

pub mod prelude {
    pub use super::simplex::prelude::*;
}
