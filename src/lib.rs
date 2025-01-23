#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg_attr(feature = "sf32lb52x", path = "./sf32lb52x/mod.rs")]
mod inner;
pub use inner::*;
