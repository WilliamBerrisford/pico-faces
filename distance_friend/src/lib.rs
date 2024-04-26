#![no_std]
#![allow(async_fn_in_trait)]

#[cfg(feature = "embedded")]
pub mod face;
#[cfg(feature = "embedded")]
pub mod utils;
