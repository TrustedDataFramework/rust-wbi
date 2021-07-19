use alloc::vec::Vec;
use crate::{to_hex, remember_bytes};
use alloc::string::*;

const ADDRESS_SIZE: usize = 20;

#[derive(Eq, Clone)]
pub struct Address {
    data: Vec<u8>
}

impl core::fmt::Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        to_hex(&self.data)
    }
}

impl Default for Address {
    fn default() -> Address {
        Address {
            data: vec![0u8; ADDRESS_SIZE]
        }
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Address) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }            
        }

        return true;      
    }    
}

impl Address {
    pub fn new(v: Vec<u8>) -> Address {
        Address {
            data: v
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn __peek(&self) -> (u64, u64){
        (self.data.as_ptr() as u64, self.data.len() as u64)
    }        

    /// should forget
    pub(crate) fn raw_clone(&self) -> Address {
        let (x, y) = (self.data.as_ptr() as u64, self.data.len() as u64);
        let v = remember_bytes(x, y);
        Address {
            data: v
        }
    }
}