// Copyright 2018 Cryptape Technology LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![cfg_attr(feature = "internal_benches", allow(unstable_features), feature(test))]
#![cfg_attr(target_arch = "wasm32", no_std)]


pub mod sm2;
pub mod sm3;
pub mod sm4;

#[macro_use]
extern crate alloc;
extern crate rand;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate yasna;
extern crate rust_wbi;

#[macro_use]
extern crate lazy_static;

pub fn seed(s: u64) {
    sm2::seed(s)
}

use alloc::string::*;
use alloc::vec::*;

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

pub(crate) fn decode_hex(data: &str) -> Vec<u8> {
    let ascii = data.as_bytes();
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

const CHARS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' ];


pub(crate) fn to_hex(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for i in data {
        s.push(CHARS[((i >> 4) & 0x0f) as usize]);
        s.push(CHARS[(i & 0x0f) as usize]);
    }
    s
}