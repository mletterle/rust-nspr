#[link(name = "nspr", vers = "0.0")];
#[feature(globs)];

pub use raw::*;

pub mod raw { pub mod nspr; }
