#![cfg_attr(target_arch = "wasm32", no_std)] // enable no_std feature in web assembly environment
#![feature(unchecked_math)] // allow unchecked math 
#[macro_use]
extern crate alloc;
extern crate core;

#[macro_use]
extern crate lazy_static;

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

pub type Bytes32 = Vec<u8>;

pub fn to_hex(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for i in data {
        s.push(CHARS[((i >> 4) & 0x0f) as usize]);
        s.push(CHARS[(i & 0x0f) as usize]);
    }
    s
}

pub fn decode_hex(data: &str) -> Vec<u8> {
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

macro_rules! forget {
    ($x: expr) => {
        alloc::boxed::Box::leak(
            alloc::boxed::Box::new($x)
        ) as *const _ as u64
    };
}

macro_rules! remember {
    ($p: expr) => {
        unsafe { *(alloc::boxed::Box::from_raw($p as *mut _)) }
    };
}

// get vector from fat pointer
// should forget 
macro_rules! to_vec {
    ($slice: expr) => {
        unsafe {
            Vec::from_raw_parts($slice.as_ptr() as *mut u8, $slice.len(), $slice.len())
        }
    };
}

pub mod wbi_type {
    pub const UINT_256: u32 = 0xec13d6d1; // keccak(uint256)
    pub const ADDRESS: u32 =  0x421683f8; // keccak(address)
    pub const STRING: u32 =   0x97fc4627; // keccak(string)
    pub const BYTES: u32 =    0xb963e9b4; // keccak(bytes)
    pub const BYTES32: u32 =  0x9878dbb4; // keccak(bytes32)
}

pub mod u256;
pub mod db;
pub mod address;
pub mod context;

use alloc::{vec::Vec};
use alloc::string::*;
use u256::U256;
use core::mem;

#[macro_export]
macro_rules! use_wbi {
    () => {                
        // Use `wee_alloc` as the global allocator.
        extern crate wee_alloc;        
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
        
        #[cfg(target_arch = "wasm32")]
        #[macro_use]
        extern crate alloc;
        #[cfg(not(target_arch = "wasm32"))]
        extern crate alloc;


        extern crate core;  
        pub use rust_wbi::{__change_t, __malloc, __peek, __malloc_256, __malloc_512};     
    };
}

#[macro_export]
macro_rules! impl_panic {
    () => {                
        #[cfg(target_arch = "wasm32")]
        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> !{
            rust_wbi::log(&format!("{:?}", info));
            unsafe { core::arch::wasm32::unreachable() }
        }      
    };
}


extern "C" {
    #[cfg(target_arch = "wasm32")]
    pub fn _log(a: u64); 
}


#[cfg(not(target_arch = "wasm32"))]
pub fn _log(a: u64) {
    let p: String = remember!(a);    
    println!("{}", p);
    mem::forget(p);
}

pub fn log(s: &str) {
    unsafe {
        let raw_cloned = String::from_raw_parts(
        s.as_ptr() as *mut u8,
      s.len(),
    s.len()
        );
        _log(forget!(raw_cloned))
    }
}

/// use vec<u8> to allocate memory, return a raw pointer, called by blockchain host
#[no_mangle]
pub fn __malloc(size: u64) -> u64 {
    let mut bytes: Vec<u8> = Vec::with_capacity(size as usize);
    unsafe {
        bytes.set_len(size as usize);
    }
    forget_bytes(bytes)
}

#[no_mangle]
pub fn __malloc_256(a0: u64, a1: u64, a2: u64, a3: u64) -> u64 {
    let u = [
        (a0 >> 32) as u32,
        a0 as u32,
        (a1 >> 32) as u32,
        a1 as u32,
        (a2 >> 32) as u32,
        a2 as u32,
        (a3 >> 32) as u32,
        a3 as u32,
    ];

    let u = U256::new(u);
    forget!(u)
}

#[no_mangle]
pub fn __malloc_512(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64 {
    let u = [
        (a0 >> 32) as u32,
        a0 as u32,
        (a1 >> 32) as u32,
        a1 as u32,
        (a2 >> 32) as u32,
        a2 as u32,
        (a3 >> 32) as u32,
        a3 as u32,
        (a4 >> 32) as u32,
        a4 as u32,
        (a5 >> 32) as u32,
        a5 as u32,
        (a6 >> 32) as u32,
        a6 as u32,
        (a7 >> 32) as u32,
        a7 as u32,        
    ];

    let u = u256::U512(u);
    forget!(u)
}


#[inline]
fn forget_bytes(t: Vec<u8>) -> u64 {
    let raw = t.as_ptr();
    let ret = (raw as usize) as u64;
    mem::forget(t);
    ret
}

// restore Vec<u8> from raw pointer and length
#[inline]
fn remember_bytes(ptr: u64, size: u64) -> Vec<u8> {
    unsafe {
        let raw = ptr as *mut u8;
        Vec::from_raw_parts(raw, size as usize, size as usize)
    }
}

#[inline]
pub fn ret<T>(d: T) -> &'static T {
    alloc::boxed::Box::leak(
        alloc::boxed::Box::new(d)
    )    
}


/// convert bytes view to rust type
#[no_mangle]
pub unsafe fn __change_t(t: u64, ptr: u64, size: u64) -> u64 {
    let v = remember_bytes(ptr, size);
    // string
    match t as u32 {
        wbi_type::STRING => forget!(String::from_utf8_unchecked(v)),
        wbi_type::BYTES | wbi_type::BYTES32 => forget!(v),
        wbi_type::UINT_256 => panic!("change_t by uint256"),
        wbi_type::ADDRESS => forget!(address::Address::new(v)),
        _ => 0
    }
}

/// __peek will convert rust type to bytes view, this function is called by host
#[no_mangle]
pub fn __peek(ptr: u64, t: u64) -> u64 {
    if t == wbi_type::STRING as u64 {
        let p: String = remember!(ptr);
        let (x, y) = (p.as_ptr() as u64, p.len());
        mem::forget(p);
        return (x << 32) | (y as u64);
    }

    if t == wbi_type::BYTES as u64 || t == wbi_type::BYTES32 as u64 {
        let p: Vec<u8> = remember!(ptr);
        let (x, y) = (p.as_ptr() as u64, p.len());
        mem::forget(p);
        return (x << 32) | (y as u64);
    }    

    if t == wbi_type::UINT_256 as u64 {
        let p: u256::U256 = remember!(ptr);
        let v = p.to_vec();
        let (x, y) = (v.as_ptr() as u64, v.len());
        mem::forget(v);
        return (x << 32) | (y as u64);
    }

    if t == wbi_type::ADDRESS as u64 {
        let p: address::Address = remember!(ptr);
        let (x, y) = p.__peek();
        mem::forget(p);
        return (x << 32) | (y as u64);
    }    
    return 0;
}


#[cfg(test)]
mod test {
    use crate::{decode_hex, u256::*};
    use crate::{to_hex};

    #[test]
    fn testx() {
        let u0: U256 = 129.into();
        let u1: U256 = 1.into();
        println!("{:?}", (u0 * &u1).u64());
    }

    #[test]
    fn test0() {
        let u0: U256 = "12345".parse().unwrap();
        let u1: U256 = "12345".parse().unwrap();
        let r: U256 = &u0 * &u1;
        println!("{:?}", r);
        println!("{:?}", to_hex(&u0.to_vec()));
        let decoded = decode_hex("0123456789abcdefABCDEF");
        let expected: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xAB, 0xCD, 0xEF];
        assert_eq!(decoded, expected);
    }
}