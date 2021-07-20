extern crate libsm;
extern crate num_bigint;

use libsm::sm2;
use libsm::sm2::signature::Signature;
use libsm::sm3::hash;
use num_bigint::BigUint;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const CHARS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' ];
const CHARS_INV: &[u8] = &[
    0,0,0,0,    0,0,0,0, // 0x08
    0,0,0,0,    0,0,0,0, // 0x10
    0,0,0,0,    0,0,0,0, // 0x18
    0,0,0,0,    0,0,0,0, // 0x20
    0,0,0,0,    0,0,0,0, // 0x28
    0,0,0,0,    0,0,0,0, // 0x30
    0,1,2,3,    4,5,6,7, // 0x38
    8,9,0,0,    0,0,0,0, // 0x40
    0,10,11,12, 13,14,15,0, // 0x48
    0,0,0,0,    0,0,0,0,  // 0x50
    0,0,0,0,    0,0,0,0, // 0x58
    0,0,0,0,    0,0,0,0, // 0x60
    0,10,11,12, 13,14,15,0 // 0x68
];

trait AsHex {
    fn to_hex(&self) -> String;
}

trait FromHex {
    fn from_hex(hex: &str) -> Self;
}

impl AsHex for Signature {
    fn to_hex(&self) -> String {
        let mut ret: Vec<u8> = vec![0; 64];
        let r = self.r_bytes();
        let s = self.s_bytes();
        &ret[(32 - r.len())..32].copy_from_slice(&r);
        &ret[(64 - s.len())..].copy_from_slice(&s);
        to_hex(&ret)
    }
}

impl FromHex for Signature {
    fn from_hex(hex: &str) -> Self {
        let rs = decode_hex(hex);
        let r = &rs[..32];
        let s = &rs[32..];
        Signature::new(r, s)
    }
}

pub fn to_hex(data: &[u8]) -> String {
    let mut s = String::with_capacity(2 + data.len() * 2);
    s.push_str("0x");
    for i in data {
        s.push(CHARS[((i >> 4) & 0x0f) as usize]);
        s.push(CHARS[(i & 0x0f) as usize]);
    }
    s
}

pub fn decode_hex(data: &str) -> Vec<u8> {
    if !data.starts_with("0x") {
        panic!("invalid bytes: {} hex bytes should starts with 0x", data);
    }

    let ascii = &data.as_bytes()[2..];
    if ascii.len() % 2 != 0 {
        panic!("odd hex string {}\n", data);
    }
    let mut r: Vec<u8> = Vec::with_capacity(data.len() / 2);
    let mut j: u8 = 0;
    let mut i: usize = 0;
    for x in ascii {
        if i % 2 != 0 {
            j = j | CHARS_INV[*x as usize];
            r.push(j);
            j = 0;
        } else {
            j = CHARS_INV[*x as usize] << 4;
        }
        i += 1;
    }
    r
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    unsafe { alert("Hello, sm-crypto!") };
}


// sm3 algorithm
#[wasm_bindgen]
pub fn sm3(x: String) -> String {
    // @pure
    let mut s = hash::Sm3Hash::new(&decode_hex(&x));
    let h = s.get_hash();
    to_hex(&h)
}

// convert private key to public key
#[wasm_bindgen]
pub fn sm2_pk_from_sk(private_key: String, compress: bool) -> String {
    // @pure
    let sig_ctx = sm2::signature::SigCtx::new();
    let ecc_ctx = sm2::ecc::EccCtx::new();
    let sk = BigUint::from_bytes_be(&decode_hex(&private_key));
    let p = sig_ctx.pk_from_sk(&sk);
    to_hex(&ecc_ctx.point_to_bytes(&p, compress))
}

// sm2 sign algorithm
#[wasm_bindgen]
pub fn sm2_sign(seed: u64, private_key: String, message: String) -> String {
    // @pure
    libsm::seed(seed);
    let c = sm2::signature::SigCtx::new();
    let sk = BigUint::from_bytes_be(&decode_hex(&private_key));
    c.sign(&decode_hex(&message), &sk, &c.pk_from_sk(&sk)).to_hex()
}

// sm2 verify
#[wasm_bindgen]
pub fn sm2_verify(seed: u64, message: String, public_key: String, sig: String) -> bool {
    // @pure
    libsm::seed(seed);
    let sig_ctx = sm2::signature::SigCtx::new();
    let s: Signature = Signature::from_hex(&sig);
    let ecc_ctx = sm2::ecc::EccCtx::new();
    let decoded = decode_hex(&public_key);
    let pk = ecc_ctx.bytes_to_point(&decoded).unwrap();
    sig_ctx.verify(&decode_hex(&message), &pk, &s)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let sk = "0xd2ea6fe0a7b0f8e520c418dca23b01f9b2f451c58fa3ed2bea55237fa451f7fd";
        let sig = super::sm2_sign(128, sk.into(), "0xff".into());
        let pk = super::sm2_pk_from_sk(sk.into(), false);
        println!("pk = {}", pk);
        println!("sig = {}", sig);
        println!("sm2 verfy");
        super::sm2_verify(128, "0xff".into(), pk, sig);
    }
}