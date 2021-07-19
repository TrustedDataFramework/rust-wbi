extern "C" {
    #[cfg(target_arch = "wasm32")]
    pub fn _context(t: u64, a: u64) -> u64;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn _context(t: u64, a: u64) -> u64 {
    0
}


use crate::{address::Address, context};
use crate::u256::U256;
use alloc::vec::Vec;
use alloc::string::*;


mod context_type {
    pub(crate) const THIS_ADDRESS: u32 = 0x644836c2; // keccak('this')
    pub(crate) const MSG_SENDER: u32 = 0xb2f2618c; // keccak('msg.sender')
    pub(crate) const MSG_VALUE: u32 = 0x6db8129b; // keccak('msg.value')
}

#[cfg(target_arch = "wasm32")]
pub fn this() -> Address {
    remember!(_context(context_type::THIS_ADDRESS as u64, 0))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn this() -> Address {
    Address::default()
}

lazy_static! {
    pub static ref msg: Msg = {
        Msg::new()
    }; 
}

#[derive(Default)]
pub struct Msg {
    pub sender: Address,
    pub value: U256
}

impl Msg {
    #[cfg(not(target_arch = "wasm32"))]    
    fn new() -> Msg {
        Msg::default()
    }

    #[cfg(target_arch = "wasm32")]    
    fn new() -> Msg {
        unsafe {
            Msg {
                sender: remember!(_context(context_type::MSG_SENDER as u64, 0)),
                value: remember!(_context(context_type::MSG_VALUE as u64, 0)),
            }
        }
    }    
}
