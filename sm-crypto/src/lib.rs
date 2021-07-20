extern crate libsm;
extern crate num_bigint;

use libsm::sm2;
use libsm::sm3::hash;
use num_bigint::BigUint;

type Bytes32 = Vec<u8>;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    unsafe { alert("Hello, sm-crypto!") };
}

#[wasm_bindgen]
pub fn sm3(x: Vec<u8>) -> Vec<u8> {
    // @pure
    let mut s = hash::Sm3Hash::new(&x);
    let h = s.get_hash();
    h.to_vec()
}

#[wasm_bindgen]
pub fn sm2_pk_from_sk(private_key: Bytes32, compress: bool) -> Vec<u8> {
    // @pure
    let sig_ctx = sm2::signature::SigCtx::new();
    let ecc_ctx = sm2::ecc::EccCtx::new();
    let sk = BigUint::from_bytes_be(&private_key);
    let p = sig_ctx.pk_from_sk(&sk);
    ecc_ctx.point_to_bytes(&p, compress)
}

#[wasm_bindgen]
pub fn sm2_sign(seed: u64, private_key: Bytes32, message: Vec<u8>) -> Vec<u8> {
    // @pure
    libsm::seed(seed);
    let c = sm2::signature::SigCtx::new();
    let sk = BigUint::from_bytes_be(&private_key);
    c.sign(&message, &sk, &c.pk_from_sk(&sk)).der_encode()
}

#[wasm_bindgen]
pub fn sm2_verify(seed: u64, message: Vec<u8>, public_key: Vec<u8>, sig: Vec<u8>) -> bool {
    // @pure
    libsm::seed(seed);
    let sig_ctx = sm2::signature::SigCtx::new();
    let s = sm2::signature::Signature::der_decode(&sig).unwrap();
    let ecc_ctx = sm2::ecc::EccCtx::new();
    sig_ctx.verify(&message, &ecc_ctx.bytes_to_point(&public_key).unwrap(), &s)
}
