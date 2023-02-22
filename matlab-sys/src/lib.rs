#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/README.md"))]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("The bindings are only valid for 64-bit applications. All Matlab versions after 2015b are only available in 64-bit.");

mod raw;
#[cfg(feature = "interleaved-complex")]
pub use raw::interleaved_complex;
#[cfg(feature = "separate-complex")]
pub use raw::separate_complex;
