#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

mod consts;

#[macro_use]
pub mod event;

pub mod bio;
pub mod cio;
pub mod dwt;
pub mod function_list;
pub mod ht_dec;
pub mod image;
pub mod invert;
pub mod j2k;
pub mod jp2;
pub mod malloc;
mod math;
pub mod mct;
pub mod mqc;
pub mod openjpeg;
pub mod pi;
pub mod sparse_array;
pub mod t1;
pub mod t1_ht_luts;
pub mod t1_luts;
pub mod t2;
pub mod tcd;
pub mod tgt;
pub mod thread;
