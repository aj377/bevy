#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! A collection of helper types and functions for working on macros within the Bevy ecosystem.

extern crate proc_macro;

mod attrs;
mod bevy_manifest;
pub mod fq_std;
mod label;
mod shape;
mod symbol;

pub use attrs::*;
pub use bevy_manifest::*;
pub use label::*;
pub use shape::*;
pub use symbol::*;
