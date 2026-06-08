#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod arc;
mod either;
#[cfg(feature = "alloc")]
mod rc;

#[cfg(feature = "alloc")]
pub use arc::ArcError;
pub use either::EitherError;
#[cfg(feature = "alloc")]
pub use rc::RcError;
