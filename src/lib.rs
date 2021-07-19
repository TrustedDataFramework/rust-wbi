#![cfg_attr(target_arch = "wasm32", no_std)]
#![feature(default_alloc_error_handler)]

use rust_wbi::log;

#[macro_use]
extern crate rust_wbi;
use_wbi!();  


#[no_mangle]
pub fn init() {
    log("hello world");
}