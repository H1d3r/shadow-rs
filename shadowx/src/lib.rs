//! # shadowx: Kernel-Level Utilities Library
//!
//! **shadowx** is a `#![no_std]` library designed for kernel operations,
//! including process management, thread handling, injection mechanisms, driver interactions,
//! registry manipulation, and more.

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

mod process;
mod thread;
mod injection;
mod module;
mod driver;
mod misc;
mod utils;
mod data;
mod offsets;

pub use process::*;
pub use thread::*;
pub use injection::*;
pub use module::*;
pub use driver::*;
pub use misc::*;
pub use utils::*;
pub use data::*;
pub use network::*;
pub use registry::*;
pub use callback::*;

pub(crate) type Result<T> = core::result::Result<T, error::ShadowError>;
