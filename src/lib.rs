#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
extern crate libc;

type __builtin_va_list = libc::c_void;

include!("ffi.rs");
