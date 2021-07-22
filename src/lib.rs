#![cfg_attr(target_arch = "wasm32", no_std)]
#![feature(default_alloc_error_handler)]

use num_bigint::BigUint;
use rust_wbi::{Bytes32, log, ret, u256::U256};
use libsm::{sm2::{self, signature::Signature}, sm3::hash};
use alloc::vec::*;

#[macro_use]
extern crate rust_wbi;
use_wbi!();  


#[no_mangle]
pub fn init() {
    log("hello crypto");
}

// sm3 algorithm
#[no_mangle]
pub fn sm3(x: Vec<u8>) -> &'static Bytes32 {
    // @pure
    let mut s = hash::Sm3Hash::new(&x);
    let h = s.get_hash();
    ret(h.to_vec())
}

// convert private key to public key
#[no_mangle]
pub fn sm2_pk_from_sk(private_key: Bytes32, compress: bool) -> &'static Vec<u8> {
    // @pure
    let sig_ctx = sm2::signature::SigCtx::new();
    let ecc_ctx = sm2::ecc::EccCtx::new();
    let sk = BigUint::from_bytes_be(&private_key);
    let p = sig_ctx.pk_from_sk(&sk);
    ret(ecc_ctx.point_to_bytes(&p, compress))
}

// sm2 verify
#[no_mangle]
pub fn sm2_verify(seed: u64, message: Vec<u8>, public_key: Vec<u8>, sig: Vec<u8>) -> bool {
    // @pure
    libsm::seed(seed);
    log("1");
    let sig_ctx = sm2::signature::SigCtx::new();
    log("2");
    let s: Signature = Signature::new(&sig[..32], &sig[32..]);
    log("3");
    let ecc_ctx = sm2::ecc::EccCtx::new();
    log("4");
    let pk = ecc_ctx.bytes_to_point(&public_key).unwrap();
    log("5");
    let r = sig_ctx.verify(&message, &pk, &s);
    log("6");
    r
}


#[no_mangle]
pub fn add(x: U256, y: U256) -> &'static U256 {
    // @pure
    ret(x + y)
}

#[no_mangle]
pub fn mul(x: U256, y: U256) -> &'static U256 {
    // @pure
    ret(x * y)
}


#[cfg(test)]
mod test {
    use rust_wbi::{decode_hex};

    #[test]
    fn test() {
        super::sm2_verify(130, decode_hex("ffff"), decode_hex("02b02ecedf61539bf9541a7064d50b7061b3dbc43d789020133c8009d8bc426912"), decode_hex("8be9e0a8d3712c090508a29602f4e82eada8717ce967f413972d2cbc9351aa87b6e82853cbfab5ca035578ee097c2d45c0b5ca8e739ab724b41810561c9cf445"));
    }
}