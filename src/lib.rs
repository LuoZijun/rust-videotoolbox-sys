#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
#![cfg(any(target_os = "macos", target_os = "ios"))]

extern crate libc;
extern crate core_foundation_sys;
extern crate coremedia_sys;
extern crate corevideo_sys;

// Document: https://developer.apple.com/documentation/videotoolbox?language=objc


pub mod base;
pub mod errors;
pub mod session;
pub mod decompression;
pub mod compression;
pub mod pixel_transfer;
pub mod multi_pass_storage;
pub mod frame_silo;
pub mod utilities;

// pub use self::base::*;
// pub use self::errors::*;
// pub use self::session::*;
// pub use self::decompression::*;
// pub use self::compression::*;
// pub use self::pixel_transfer::*;
// pub use self::multi_pass_storage::*;
// pub use self::frame_silo::*;
// pub use self::utilities::*;