#[feature(globs)];
#[link(name = "nspr", vers = "0.0")]

pub use raw::*;

pub mod raw { pub mod nspr; }
