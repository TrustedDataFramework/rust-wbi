#![cfg_attr(target_arch = "wasm32", no_std)]
#![feature(default_alloc_error_handler)]

use num_bigint::BigUint;
use rust_wbi::{Bytes32, log, ret};
use libsm::{sm2, sm3::hash};
use alloc::vec::*;

#[macro_use]
extern crate rust_wbi;
use_wbi!();  


#[no_mangle]
pub fn init() {
    log("hello crypto");
}

#[no_mangle]
pub fn sm3(x: Vec<u8>) -> &'static Bytes32 { // @pure
    let mut s = hash::Sm3Hash::new(&x);
    let h = s.get_hash();
    ret(h.to_vec())
}

#[no_mangle]
pub fn sm2_pk_from_sk(private_key: Bytes32, compress: bool) -> &'static Vec<u8> { // @pure
    let sig_ctx = sm2::signature::SigCtx::new();
    let ecc_ctx = sm2::ecc::EccCtx::new();
    let sk = BigUint::from_bytes_be(&private_key);
    let p = sig_ctx.pk_from_sk(&sk);
    ret(ecc_ctx.point_to_bytes(&p, compress))
}

#[no_mangle]
pub fn sm2_verify(seed: u64, message: Vec<u8>, public_key: Vec<u8>, sig: Vec<u8> ) -> bool { // @pure
    libsm::seed(seed);
    let sig_ctx = sm2::signature::SigCtx::new();
    let s = sm2::signature::Signature::der_decode(&sig).unwrap();
    let ecc_ctx = sm2::ecc::EccCtx::new();
    sig_ctx.verify(&message, &ecc_ctx.bytes_to_point(&public_key).unwrap(), &s)
} 

#[cfg(test)]
mod test {
    use rust_wbi::{decode_hex, to_hex};

    #[test]
    fn test() {
        println!("{}", to_hex(super::sm3(decode_hex("0000"))))
    }
}