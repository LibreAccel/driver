#![no_std]
#![feature(untagged_unions)]

pub mod base;

#[cfg(feature = "intrin")]
pub mod intrin;
#[cfg(feature = "netio")]
pub mod netio;
#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

pub use cty::*;
