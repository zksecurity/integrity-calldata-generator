#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod first_layer;
pub mod formula;
pub mod fri;
pub mod group;
pub mod last_layer;
pub mod layer;
pub mod types;

pub use crate::fri::{CONST_STATE, VAR_STATE, WITNESS};

#[cfg(any(test, feature = "test_fixtures"))]
pub mod fixtures;
#[cfg(test)]
pub mod tests;
