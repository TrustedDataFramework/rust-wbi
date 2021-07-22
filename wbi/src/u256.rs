use alloc::string::*;
use alloc::vec::Vec;
use core::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use core::{
    ops::{Add, Div, Mul, Rem, Sub},
    str::FromStr,
};

trait ToU256 {
    fn to_u256(&self) -> [u32; U256_MAGS];
}

trait AsU256 {
    fn as_u256(&self) -> &[u32];
}

impl ToU256 for [u32; U512_MAGS] {
    fn to_u256(&self) -> [u32; U256_MAGS] {
        let mut out = ZEROS;
        out.copy_from_slice(&self[U256_MAGS..]);
        out
    }
}

impl AsU256 for [u32; U512_MAGS] {
    fn as_u256(&self) -> &[u32] {
        &self[U256_MAGS..]
    }
}

macro_rules! trim_zeros {
    ($slice: ident) => {{
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
    }};
}

const U256_MAGS: usize = 8;
const U512_MAGS: usize = 16;
const ZEROS: [u32; U256_MAGS] = [0; U256_MAGS];

const MAX_U256: [u32; U256_MAGS] = [
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
];

const CHARS: &'static str = "0123456789";

extern "C" {
    #[cfg(target_arch = "wasm32")]
    pub fn _u256(
        op: u64,
        l0: u64,
        l1: u64,
        l2: u64,
        l3: u64,
        r0: u64,
        r1: u64,
        r2: u64,
        r3: u64,
    ) -> u64;
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn __u256(op: u64, l0: u64, l1: u64, l2: u64, l3: u64, r0: u64, r1: u64, r2: u64, r3: u64) -> u64 {
    unsafe { _u256(op, l0, l1, l2, l3, r0, r1, r2, r3) }
}

#[cfg(not(target_arch = "wasm32"))]
fn _u256(op: u64, l0: u64, l1: u64, l2: u64, l3: u64, r0: u64, r1: u64, r2: u64, r3: u64) -> u64 {
    macro_rules! as_u256 {
        ($l0: ident, $l1: ident, $l2: ident, $l3: ident) => {{
            [
                ($l0 >> 32) as u32,
                $l0 as u32,
                ($l1 >> 32) as u32,
                $l1 as u32,
                ($l2 >> 32) as u32,
                $l2 as u32,
                ($l3 >> 32) as u32,
                $l3 as u32,
            ]
        }};
    }

    let l_buf = as_u256!(l0, l1, l2, l3);
    let r_buf = as_u256!(r0, r1, r2, r3);

    let out = match op as u32 {
        u256_op::SUM => forget!(U512(primitive::add(&l_buf, &r_buf))),
        u256_op::SUB => forget!(U256(primitive::sub(&l_buf, &r_buf))),
        u256_op::MUL => forget!(U512(primitive::mul(&l_buf, &r_buf))),
        u256_op::DIV => forget!(U256(primitive::div_mod(&l_buf, &r_buf).0)),
        u256_op::MOD => forget!(U256(primitive::div_mod(&l_buf, &r_buf).1)),
        _ => panic!(),
    };

    out
}

#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn __u256(op: u64, l0: u64, l1: u64, l2: u64, l3: u64, r0: u64, r1: u64, r2: u64, r3: u64) -> u64 {
    _u256(op, l0, l1, l2, l3, r0, r1, r2, r3)
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

mod u256_op {
    pub const SUM: u32 = 0;
    pub const SUB: u32 = 1;
    pub const MUL: u32 = 2;
    pub const DIV: u32 = 3;
    pub const MOD: u32 = 4;
}

#[derive(Clone, Eq, Ord)]
pub struct U256(pub [u32; U256_MAGS]);
pub struct U512(pub [u32; U512_MAGS]);

impl U512 {
    pub fn to_u256(&self) -> U256 {
        let mut o: [u32; U256_MAGS] = [0; U256_MAGS];
        o.copy_from_slice(&self.0[U256_MAGS..]);
        return U256(o);
    }

    pub fn u64(&self) -> u64 {
        ((self.0[U512_MAGS - 2] as u64) << 32) | (self.0[U512_MAGS - 1] as u64)
    }
}

impl core::fmt::Display for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&to_string(self))
    }
}

impl core::fmt::Debug for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&to_string(self))
    }
}

impl PartialEq for U256 {
    fn eq(&self, other: &U256) -> bool {
        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        return true;
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &U256) -> Option<Ordering> {
        for i in 0..self.0.len() {
            if self.0[i] > other.0[i] {
                return Some(Ordering::Greater);
            }
            if self.0[i] < other.0[i] {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

fn to_string(s: &U256) -> String {
    let mut ret = String::new();

    let base: U256 = 10u64.into();

    let mut n = s.clone();

    if n.is_zero() {
        return "0".to_string();
    }
    while !n.is_zero() {
        let div = &n / &base;
        let m = &n % &base;
        n = div;
        ret.insert(0, CHARS.as_bytes()[m.u64() as usize] as char);
    }
    return ret;
}

fn add_overflow(_: &U256, _: &U256, o: &U512) -> bool {
    o.0[U256_MAGS - 1] != 0
}

fn sub_overflow(left: &U256, right: &U256, _: &U256) -> bool {
    right > left
}

fn mul_overflow(left: &U256, right: &U256, z: &U512) -> bool {
    !is_zero(&z.0[..U256_MAGS])
}

fn div_overflow(left: &U256, right: &U256, z: &U256) -> bool {
    right.is_zero()
}

macro_rules! call_u256 {
    ($op: expr, $l: ident, $r: ident) => {
        __u256(
            ($op) as u64,
            ($l.0[0] as u64) << 32 | ($l.0[1] as u64),
            ($l.0[2] as u64) << 32 | ($l.0[3] as u64),
            ($l.0[4] as u64) << 32 | ($l.0[5] as u64),
            ($l.0[6] as u64) << 32 | ($l.0[7] as u64),
            ($r.0[0] as u64) << 32 | ($r.0[1] as u64),
            ($r.0[2] as u64) << 32 | ($r.0[3] as u64),
            ($r.0[4] as u64) << 32 | ($r.0[5] as u64),
            ($r.0[6] as u64) << 32 | ($r.0[7] as u64),
        );
    };
}

// overflow check
macro_rules! impl_op {
    ($tr: ident, $fn: ident, $op: expr, $out: ident, $overflow: ident) => {
        impl<'a> $tr for &'a U256 {
            type Output = U256;

            fn $fn(self, rhs: &'a U256) -> U256 {
                let p = call_u256!($op, self, rhs);
                let o: $out = remember!(p);
                if $overflow(self, &rhs, &o) {
                    panic!("math overflow for op {}", $op);
                }
                o.into()
            }
        }

        impl<'a> $tr<U256> for &'a U256 {
            type Output = U256;

            fn $fn(self, rhs: U256) -> U256 {
                let p = call_u256!($op, self, rhs);
                let o: $out = remember!(p);
                if $overflow(self, &rhs, &o) {
                    panic!("math overflow for op {}", $op);
                }                
                o.into()
            }
        }

        impl<'a> $tr<&'a U256> for U256 {
            type Output = U256;

            fn $fn(self, rhs: &'a U256) -> U256 {
                let p = call_u256!($op, self, rhs);
                let o: $out = remember!(p);
                if $overflow(&self, &rhs, &o) {
                    panic!("math overflow for op {}", $op);
                }                
                o.into()
            }
        }

        impl $tr for U256 {
            type Output = U256;

            fn $fn(self, rhs: U256) -> U256 {
                let p = call_u256!($op, self, rhs);
                let o: $out = remember!(p);
                if $overflow(&self, &rhs, &o) {
                    panic!("math overflow for op {}", $op);
                }
                o.into()
            }
        }
    };
}

impl_op!(Add, add, u256_op::SUM, U512, add_overflow);
impl_op!(Sub, sub, u256_op::SUB, U256, sub_overflow);
impl_op!(Mul, mul, u256_op::MUL, U512, mul_overflow);
impl_op!(Div, div, u256_op::DIV, U256, div_overflow);
impl_op!(Rem, rem, u256_op::MOD, U256, div_overflow);

impl From<U512> for U256 {
    fn from(o: U512) -> U256 {
        let mut v = ZEROS;
        v.copy_from_slice(&o.0[U256_MAGS..]);
        U256(v)
    }
}

impl From<u64> for U256 {
    fn from(o: u64) -> U256 {
        let mut mag = ZEROS;
        mag[7] = o as u32;
        mag[6] = (o >> 32) as u32;
        U256(mag)
    }
}

impl U256 {
    pub fn max() -> U256 {
        U256(MAX_U256)
    }

    pub fn pow(&self, o: u64) -> U256 {
        let mut ret = U256::one();

        for _ in 0..o {
            ret = &ret * self;
        }
        ret
    }

    pub fn from_mag(mag: &[u32]) -> U256 {
        assert!(mag.len() <= U256_MAGS, "u256 overflow");
        let mut d = ZEROS;
        d[U256_MAGS - mag.len()..].copy_from_slice(mag);
        U256(d)
    }

    pub fn new(data: [u32; U256_MAGS]) -> U256 {
        U256(data)
    }

    pub(crate) fn to_vec(&self) -> Vec<u8> {
        let b = self.bytes32();
        trim_zeros!(b)
    }

    pub fn zero() -> U256 {
        U256(ZEROS)
    }

    pub fn one() -> U256 {
        let mut o = ZEROS;
        o[U256_MAGS - 1] = 1;
        U256(o)
    }

    pub fn u64(&self) -> u64 {
        ((self.0[U256_MAGS - 2] as u64) << 32) | (self.0[U256_MAGS - 1] as u64)
    }

    pub fn is_zero(&self) -> bool {
        for i in 0..U256_MAGS {
            if self.0[i] != 0 {
                return false;
            }
        }
        true
    }

    pub fn bytes32(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..U256_MAGS {
            v.extend_from_slice(&self.0[i].to_be_bytes())
        }
        v
    }

    pub fn checked_add(&self, other: &U256) -> U512 {
        let p = call_u256!(u256_op::SUM, self, other);
        let o: U512 = remember!(p);
        o         
    }

    pub fn checked_mul(&self, other: &U256) -> U512 {
        let p = call_u256!(u256_op::MUL, self, other);
        let o: U512 = remember!(p);
        o         
    }    
}

pub(crate) fn is_zero(x: &[u32]) -> bool {
    for i in x.iter().rev() {
        if *i != 0 {
            return false;
        }
    }
    true
}

#[cfg(not(target_arch = "wasm32"))]
mod primitive {
    use super::{AsU256, ToU256};
    use super::{U256_MAGS, U512_MAGS, ZEROS};

    pub(crate) fn add(l: &[u32], r: &[u32]) -> [u32; U512_MAGS] {
        let mut carry: u64 = 0;
        let mut out: [u32; U512_MAGS] = [0; U512_MAGS];

        for i in (0..U256_MAGS).rev() {
            let added = (l[i] as u64) + (r[i] as u64) + carry as u64;
            out[U256_MAGS + i] = added as u32;
            carry = added >> 32;
        }
        out
    }

    pub(crate) fn sub(l: &[u32], r: &[u32]) -> [u32; U256_MAGS] {
        let mut neg = ZEROS;

        for i in 0..neg.len() {
            neg[i] = !r[i];
        }

        let mut one = ZEROS;
        one[super::U256_MAGS - 1] = 1;
        let mut tmp = ZEROS;
        // tmp = 1 + neg(right)
        tmp.copy_from_slice(add(&neg, &one).as_u256());
        // tmp = left + (1 + neg(right))
        let mut out = ZEROS;
        out.copy_from_slice(add(l, &tmp).as_u256());
        out
    }

    pub(crate) fn mul(l: &[u32], r: &[u32]) -> [u32; U512_MAGS] {
        let x = trim_zeros!(l);
        let y = trim_zeros!(r);
        if x.len() == 0 || y.len() == 0 {
            return [0; U512_MAGS];
        }

        let z = uncheck_mul(&x, &y);
        let mut out: [u32; U512_MAGS] = [0; U512_MAGS];
        out[U512_MAGS - z.len()..].copy_from_slice(&z);
        out
    }

    // perform multiplication on non-zero, non zero prefixed mag
    pub(crate) fn uncheck_mul(x: &[u32], y: &[u32]) -> Vec<u32> {
        let x_len = x.len();
        let y_len = y.len();
        let mut z: Vec<u32> = vec![0; x_len + y_len];

        let mut carry: u64 = 0;
        let mut j = (y_len - 1) as i32;
        let mut k = (y_len + x_len - 1) as i32;

        while j >= 0 {
            let product = (y[j as usize] as u64) * (x[x_len - 1] as u64) + carry;
            z[k as usize] = product as u32;
            carry = product >> 32;
            j -= 1;
            k -= 1;
        }

        z[x_len - 1] = carry as u32;
        let mut i = (x_len as i32) - 2;

        while i >= 0 {
            carry = 0;
            j = (y_len as i32) - 1;
            k = (y_len as i32) + i;

            while j >= 0 {
                let product = (y[j as usize] as u64) * (x[i as usize] as u64)
                    + (z[k as usize] as u64)
                    + carry;
                z[k as usize] = product as u32;
                carry = product >> 32;
                j -= 1;
                k -= 1;
            }

            z[i as usize] = carry as u32;
            i -= 1;
        }

        z
    }

    fn one_lshift(i: usize) -> [u32; U256_MAGS] {
        let count = i / 32;
        let offset = i % 32;
        let mut r = ZEROS;
        r[7 - count] = 1 << offset;
        r
    }

    struct MutMag<'a>(&'a mut [u32]);

    impl<'a> MutMag<'a> {
        fn right_shift(&'a mut self) {
            for i in (0..self.0.len()).rev() {
                self.0[i] = self.0[i] >> 1;
                if i >= 1 {
                    self.0[i] |= (self.0[i - 1] & 0x01) << 31
                }
            }
        }
    }

    fn right_shift_n(x: &[u32], n: u32) -> [u32; U256_MAGS] {
        let mut r = ZEROS;
        let c = (n / 32) as usize;
        let off = (n % 32) as usize;

        for i in 0..U256_MAGS {
            let i = i as usize;
            if i + c < U256_MAGS {
                r[i + c] = x[i];
            }
        }

        if off == 0 {
            return r;
        }

        for i in (0..U256_MAGS).rev() {
            r[i] = r[i] >> off;
            if i > 0 {
                r[i] |= (r[i - 1] & (!(0xffffffffu32 << off))) << (32 - off);
            }
        }

        r
    }

    pub(crate) fn div_mod(x: &[u32], y: &[u32]) -> ([u32; U256_MAGS], [u32; U256_MAGS]) {
        assert!(!super::is_zero(y), "divided by zero");

        let mut quo = ZEROS;

        let divisor = {
            let mut d = ZEROS;
            d.copy_from_slice(y);
            d
        };

        let mut dividend = {
            let mut d = ZEROS;
            d.copy_from_slice(x);
            d
        };

        let mut div = [0u32; U512_MAGS];
        div[..U256_MAGS].copy_from_slice(y);

        for i in (0..256).rev() {
            let mut d = MutMag(&mut div);
            d.right_shift();
            if cmp(&right_shift_n(&dividend, i), &divisor) >= 0 {
                quo = add(&quo, &one_lshift(i as usize)).to_u256();
                dividend = {
                    let to_sub = &div[U256_MAGS..];
                    assert!(cmp(&dividend, &to_sub) >= 0, "divided overflow");
                    sub(&dividend, &to_sub)
                }
            }

        }

        (quo, dividend)
    }

    fn cmp(x: &[u32], y: &[u32]) -> i32 {
        for i in 0..U256_MAGS {
            if x[i] > y[i] {
                return 1;
            }
            if x[i] < y[i] {
                return -1;
            }
        }
        return 0;
    }
}
