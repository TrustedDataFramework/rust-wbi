macro_rules! trim_zeros {
    ($slice: ident) => {
        {
            let mut i: usize = $slice.len();

            for (j, u) in $slice.iter().enumerate() {
                if *u != 0 {
                    i = j;
                    break;
                } 
            }
    
            let mut v = alloc::vec::Vec::with_capacity($slice.len() - i);
            v.extend_from_slice(&$slice[i..]);
            v          
        }
    };
}

use core::{ops::{Add, Sub, Mul, Div, Rem}, str::FromStr};
use core::cmp::{PartialEq, Eq, PartialOrd, Ordering};
use alloc::{vec::Vec};
use crate::{remember_bytes};
use alloc::string::*;

const CHARS: &'static str = "0123456789";

extern "C" {
    #[cfg(target_arch = "wasm32")]
    pub fn _u256(op: u64, left: u64, right: u64) -> u64;
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn __u256(op: u64, left: u64, right: u64) -> u64 {
    unsafe { _u256(op, left, right) }
}

#[cfg(not(target_arch = "wasm32"))]  
fn _u256(op: u64, left: u64, right: u64) -> u64 {
    let l: U256 = remember!(left);
    let r: U256 = remember!(right);
    let l_buf = primitive::from_u256(&l);
    let r_buf = primitive::from_u256(&r);

    let out = 
    match op {
        0 => primitive::add(&l_buf, &r_buf),
        1 => primitive::sub(&l_buf, &r_buf),
        2 => primitive::mul(&l_buf, &r_buf),  
        3 => primitive::div_mod(&l_buf, &r_buf).0,  
        4 => primitive::div_mod(&l_buf, &r_buf).1,
        _ => panic!()
    };

    core::mem::forget(l);
    core::mem::forget(r);
    let o = primitive::to_u256(&out);
    forget!(o)
}

#[cfg(not(target_arch = "wasm32"))]  
#[inline]
fn __u256(op: u64, left: u64, right: u64) -> u64 {
    _u256(op, left, right)
}

impl Default for U256 {
    fn default() -> U256 {
        U256::zero()
    }
}

impl FromStr for U256 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut r = U256::zero();
        let mut base = U256::one();
        let ten = U256::from(10 as u64);

        for c in s.as_bytes().iter().rev() {
            let n = *c - b'0';
            let n = U256::from(n as u64);
            r = &r + &(&n * &base);
            base = &base * &ten;
        }
        return Ok(r);   
    }
}

enum Op {
    SUM = 0,
    SUB = 1,
    MUL = 2,
    DIV = 3,
    MOD = 4   
}

#[derive(Clone, Eq, Ord)]
pub struct U256 {
    data: Vec<u8>,
}

impl core::fmt::Debug for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for U256 {
    fn eq(&self, other: &U256) -> bool {
        if self.data.len() > other.data.len() {
            return false;
        }
        if self.data.len() < other.data.len() {
            return false;
        }        

        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }            
        }

        return true;      
    }    
}

impl PartialOrd for U256 {

    fn partial_cmp(&self, other: &U256) -> Option<Ordering> {
        if self.data.len() > other.data.len() {
            return Some(Ordering::Greater);
        }
        if self.data.len() < other.data.len() {
            return Some(Ordering::Less);
        }        

        for i in 0..self.data.len() {
            if self.data[i] > other.data[i] {
                return Some(Ordering::Greater);
            }
            if self.data[i] < other.data[i] {
                return Some(Ordering::Less);
            }            
        }

        Some(Ordering::Equal)
    }
}


impl ToString for U256 {
    fn to_string(&self) -> String {
        let mut ret = String::new();

        let base: U256 = 10u64.into();

        let mut n = self.clone();

        if n.is_zero() {
            return "0".to_string()
        }
        while !n.is_zero() {
            let div = &n / &base;
            let m = &n % &base;
            n = div;
            ret.insert(0, CHARS.as_bytes()[m.u64() as usize] as char);
        }
        return ret
    }
}

// overflow check
macro_rules! impl_op {
    ($tr: ident, $fn: ident, $op: expr, $overflow: ident) => {
        impl<'a> $tr for &'a U256 {
            type Output = U256;    
        
            fn $fn(self, rhs: &'a U256) -> U256 {
                let p = __u256($op as u64, forget!(self.raw_clone()), forget!(rhs.raw_clone()));
                let o: U256 = remember!(p);
                if $overflow(self, rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }
                o
            }
        }

        impl<'a> $tr<U256> for &'a U256 {
            type Output = U256;    
        
            fn $fn(self, rhs: U256) -> U256 {
                let p = __u256($op as u64, forget!(self.raw_clone()), forget!(rhs.raw_clone()));
                let o = remember!(p);
                if $overflow(self, &rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }                
                o
            }
        }        
        
        impl<'a> $tr<&'a U256> for U256 {
            type Output = U256;  
        
            fn $fn(self, rhs: &'a U256) -> U256 {     
                let p = __u256($op as u64, forget!(self.raw_clone()), forget!(rhs.raw_clone()));
                let o: U256 = remember!(p);
                if $overflow(&self, rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }                   
                o
            }
        }     
        
        impl $tr for U256 {
            type Output = U256;  
        
            fn $fn(self, rhs: U256) -> U256 {              
                let p = __u256($op as u64, forget!(self.raw_clone()), forget!(rhs.raw_clone()));
                let o: U256 = remember!(p);
                if $overflow(&self, &rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }                   
                o
            }
        }          
    };
}

fn add_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    out < left || out < right
}

fn sub_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    right > left
}

fn mul_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    if left.is_zero() {
        false
    } else {
        &(out / left) != right
    }
}

fn div_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    right.is_zero()
}

impl_op!(Add, add, Op::SUM, add_over_flow);
impl_op!(Sub, sub, Op::SUB, sub_over_flow);
impl_op!(Mul, mul, Op::MUL, mul_over_flow);
impl_op!(Div, div, Op::DIV, div_over_flow);
impl_op!(Rem, rem, Op::MOD, div_over_flow);


impl From<u64> for U256 {
    fn from(o: u64) -> U256 {
        let bytes: [u8; 8] = o.to_be_bytes();
        U256::new(trim_zeros!(bytes))
    }
}

impl U256 {
    // should forget
    pub(crate) fn raw_clone(&self) -> U256{
        let (x, y) = (self.data.as_ptr() as u64, self.data.len() as u64);
        let v = remember_bytes(x, y);
        U256 {
            data: v
        }
    }

    pub fn max() -> U256 {
        U256 {
            data: vec![0xffu8; 32]
        }
    }

    pub fn pow(&self, o: u64) -> U256 {
        let mut ret = U256::one();
    
        for _ in 0..o{
            ret = &ret * self
        }
        ret      
    }

    pub fn new(v: Vec<u8>) -> U256 {
        U256 {
            data: v
        }        
    }  

    pub(crate) fn __peek(&self) -> (u64, u64){
        (self.data.as_ptr() as u64, self.data.len() as u64)
    }    

    pub(crate) fn as_slice(&self) -> &[u8]{
        &self.data
    }       

    pub fn zero() -> U256 {
        U256 {
            data: Vec::new()
        }
    }

    pub fn one() -> U256 {
        U256 {
            data: vec![1u8]
        }
    }

    pub fn u64(&self) -> u64 {
        let mut v = [0u8; 8];
        let i = self.data.len();
        (&mut v[8-i..]).copy_from_slice(&self.data);
        u64::from_be_bytes(v)
    }


    pub fn is_zero(&self) -> bool {
        self.data.len() == 0
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(not(target_arch = "wasm32"))]  
mod primitive {
    // overflow check
    pub(crate) fn is_overflow(x: u64, y: u64, add_one: bool) -> bool {
        let highest_bit = 1u64 << 63;
        let mask = !highest_bit;
        (highest_bit & x != 0 && highest_bit & y != 0)
        ||
        (
            (highest_bit & x != 0 || highest_bit & y != 0) 
            &&
            (x & mask) + (y & mask) + (add_one as u64) >= highest_bit
        )
    }

    pub(crate) fn from_u256(x: &super::U256) -> [u64; 4] {
        let mut out: [u64; 4] = [0u64; 4];
        let mut tmp: [u8; 32] = [0u8; 32];
        tmp[(32 - x.as_slice().len())..].copy_from_slice(x.as_slice());
        
        for i in 0..out.len() {
            let mut be = [0u8; 8];
            be.copy_from_slice(&tmp[i*8..i*8 + 8]);
            out[i] = u64::from_be_bytes(be);
        }

        out
    }

    pub(crate) fn to_u256(x: &[u64]) -> super::U256{
        let mut tmp: [u8; 32] = [0u8; 32];

        for i in 0..x.len() {
            let be = x[i].to_be_bytes();
            tmp[i*8..i*8 + 8].copy_from_slice(&be);
        }

        super::U256::new(trim_zeros!(tmp))
    }

    pub(crate) fn add(l: &[u64], r: &[u64]) -> [u64; 4] {
        let mut overflow: bool = false;
        let mut out: [u64; 4] = [0u64; 4];

        for i in (0usize..4usize).rev() {
            out[i] = unsafe { l[i].unchecked_add(r[i]).unchecked_add(overflow as u64) };
            overflow = is_overflow(l[i], r[i], overflow);
        }
        out
    }    

    pub(crate) fn sub(l: &[u64], r: &[u64]) -> [u64; 4] {
        let mut neg = [0u64; 4];
    
        for i in 0..neg.len() {
            neg[i] = !r[i];
        }
    
        let mut one = [0u64; 4];
        one[3] = 1;
    
        add(l, &add(&neg, &one))
    }    

    pub(crate) fn is_zero(x: &[u64]) -> bool{
        for i in x.iter().rev() {
            if *i != 0 {
                return false;
            }
        }
        true
    }

    macro_rules! last_bit {
        ($slice: expr) => {
            $slice[3] & 1 != 0
        };
    }    

    fn lshift(u: &mut [u64]) {
        let highest_bit = 1u64 << 63;
    
        for i in 0..u.len() {
            u[i] = u[i] << 1;
            if  i + 1 < u.len() {
                u[i] |= (u[i + 1] & highest_bit) >> 63;
            }        
        }    
    }
    
    fn rshift(u: &mut [u64]) {    
        for i in (0..u.len()).rev() {
            u[i] = u[i] >> 1;
            if i >= 1 {
                u[i] |= (u[i - 1] & 0x01) << 63
            }        
        }
    }

    pub(crate) fn mul(_l: &[u64], _r: &[u64]) -> [u64; 4]{
        let mut ret = [0u64; 4];
        let mut r = [0u64; 4];
        let mut l = [0u64; 4];    
        r.copy_from_slice(_r);
        l.copy_from_slice(_l);
    
        while !is_zero(&r) {
            let n = last_bit!(&r);
            if n {
                ret = add(&ret, &l);
            }
            lshift(&mut l);
            rshift(&mut r);
        }
        ret
    }   
    
    fn one_lshift(i: usize) -> [u64; 4] {
        let count = i / 64;
        let offset = i % 64;
        let mut r = [0u64; 4];
        r[3 - count] = 1 << offset;
        r
    }
    
    fn lshift_n(_l: &[u64], n: usize) -> [u64; 4] {
    
        let count = n / 64;
        let offset = n % 64;
        let mut r = [0u64; 4];
    
        let mut u = [0u64; 4];
        u.copy_from_slice(_l);
    
    
        for i in (0..4).rev() {
            if i >= count {
                let j = i - count; // i = j + count 
                let m = u[i] << offset; // r[j] |= u[j+count] << offset
                r[j] |= m;
            }      
            
            if i > count && offset != 0{
                let j = i - count;
                let mask = 0xffffffffffffffff << (64 - offset);
                let x =  (u[i] & mask) >> (64 - offset);
                r[j - 1] |= x; // r[j - 1] |= (u[j + count] & mask) >> (64 - offset)
            }        
        }
        return r
    }
    
    fn rshift_n(_l: &[u64], n: usize) -> [u64; 4] {
    
        let count = n / 64;
        let offset = n % 64;
    
        let mut r = [0u64; 4];
        let mut u = [0u64; 4];
        u.copy_from_slice(_l);
    
        for i in 0..u.len() {
            let j = i + count;
            if j < r.len() {
                let m = u[i] >> offset;
                r[j] |= m;
            }   
            if  j + 1 < r.len() && offset != 0{
                let mask = 0xffffffffffffffff >> (64 - offset);
                let x = (u[i] & mask) << (64 - offset);
                r[j + 1] |= x;
            }             
        }
        r
    } 
    
    pub(crate) fn div_mod(_l: &[u64], _r: &[u64]) -> ([u64; 4], [u64;4]) {
        assert!(!is_zero(_r), "divided by zero");
    
        let mut quo = [0u64; 4];
        let mut divisor = [0u64; 4];
        let mut dividend = [0u64; 4];    
        divisor.copy_from_slice(_r);
        dividend.copy_from_slice(_l);
    
        for i in (0..256).rev() {
            if cmp(&rshift_n(&dividend, i), &divisor) >= 0 {
                quo = add(&quo, &one_lshift(i));
                dividend =  {
                    let to_sub = lshift_n(&divisor, i);
                    assert!(cmp(&dividend, &to_sub) >= 0, "divided overflow");
                    sub(&dividend, &to_sub)
                }
            }
        }
    
        (quo, dividend)
    }    

    fn cmp(_l: &[u64], _r: &[u64]) -> i32 {
        for i in 0.._l.len() {
            if _l[i] > _r[i] {
                return 1;
            }
            if _l[i] < _r[i] {
                return -1;
            }        
        }
        return 0;
    }    
}