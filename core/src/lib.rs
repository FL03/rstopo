/*
    Appellation: rstopo-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstopo
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod ops;
pub mod traits;
pub mod types;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::error::prelude::*;
    pub use crate::ops::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
}
