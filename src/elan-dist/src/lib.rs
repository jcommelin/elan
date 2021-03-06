#![recursion_limit = "1024"]

extern crate regex;
extern crate itertools;
extern crate walkdir;
extern crate toml;
extern crate flate2;
extern crate tar;
extern crate url;
extern crate elan_utils;
#[macro_use]
extern crate error_chain;
extern crate sha2;
extern crate json;
extern crate zip;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;
#[cfg(not(windows))]
extern crate libc;

pub use errors::*;
pub use notifications::{Notification};

pub mod temp;

pub mod dist;
pub mod errors;
pub mod notifications;
pub mod prefix;
mod component;
mod manifestation;
pub mod download;
pub mod manifest;
pub mod config;
