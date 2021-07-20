#![cfg_attr(target_arch = "wasm32", no_std)]
#![feature(default_alloc_error_handler)]

use rust_wbi::{log, u256::U256, ret};
use libsm::sm3::hash;

#[macro_use]
extern crate rust_wbi;
use_wbi!();  


#[no_mangle]
pub fn init() {
    log("hello crypto");
}

#[no_mangle]
pub fn sm3(x: U256) -> &'static U256 {
    let mut s = hash::Sm3Hash::new(&x.bytes32());
    let h = s.get_hash();
    ret(U256::from_slice(&h))
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("{}", super::sm3("1".parse().unwrap()))
    }
}