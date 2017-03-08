#![allow(non_camel_case_types)]

extern crate fftw_sys as ffi;
extern crate num_traits;
#[macro_use]
extern crate lazy_static;

pub mod pair;
pub mod complex;
pub mod r2r;
pub mod raw_vec;
pub mod plan;
pub mod enums;
mod util;

pub use pair::*;
pub use complex::c64;
pub use enums::*;
