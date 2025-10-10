//! Kernel-Level Utilities Library

#![no_std]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]
#![allow(non_snake_case)]

extern crate alloc;

/// Port communication utilities.
pub mod network;

/// Error handling utilities.
pub mod error;

/// Registry manipulation utilities.
pub mod registry;

/// Kernel callback management.
pub mod callback;

mod data;
mod driver;
mod injection;
mod misc;
mod module;
mod offsets;
mod process;
mod thread;
mod utils;

pub use data::*;
pub use driver::*;
pub use injection::*;
pub use misc::*;
pub use module::*;
pub use network::*;
pub use process::*;
pub use registry::*;
pub use thread::*;
pub use utils::*;
