#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

// See https://github.com/rust-lang/pin-utils/pull/26#discussion_r344491597
#[allow(unused_extern_crates)]
extern crate pin_project_lite as core;

// Dummy module to check that the expansion refers to the crate.
mod pin_project_lite {}

use ::pin_project_lite::pin_project;

pin_project! {
    struct StructDefault<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

#[test]
fn test() {}
