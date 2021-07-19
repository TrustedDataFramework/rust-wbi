use alloc::vec::Vec;

enum Op {
    SET = 0, GET = 1, REMOVE = 2, HAS = 3
}

pub fn insert(key: &[u8], value: &[u8]) {
    let k = to_vec!(key);
    let v = to_vec!(value);
    __db(Op::SET as u64, forget!(k), forget!(v));
}

pub fn contains_key(key: &[u8]) -> bool {
    let k = to_vec!(key);
    __db(Op::HAS as u64, forget!(k), 0) != 0
}

pub fn get(key: &[u8]) -> Option<Vec<u8>>{
    let k = to_vec!(key);
    if !contains_key(key) {
        core::mem::forget(k);
        Option::None
    } else {
        let p = __db(Op::GET as u64, forget!(k), 0);
        Some(remember!(p))
    }
}

pub fn remove(key: &[u8]) {
    let k = to_vec!(key);
    __db(Op::REMOVE as u64, forget!(k), 0);
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn __db(op: u64, left: u64, right: u64) -> u64 {
    unsafe { _db(op, left, right ) }
}

#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn __db(op: u64, left: u64, right: u64) -> u64 {
    _db(op, left, right )
}

extern "C" {
    #[cfg(target_arch = "wasm32")]
    pub fn _db(op: u64, left: u64, right: u64) -> u64;
}

#[cfg(not(target_arch = "wasm32"))]
lazy_static! {
    static ref _mem_db: std::sync::RwLock<std::collections::BTreeMap<Vec<u8>, Vec<u8>>> = std::sync::RwLock::new(std::collections::BTreeMap::new());
}


#[cfg(not(target_arch = "wasm32"))]
pub fn _db(op: u64, left: u64, right: u64) -> u64 {
    let k: Vec<u8> = remember!(left);
    
    let ret = match op {
        0 => {
            let v: Vec<u8> = remember!(right);
            _mem_db.write().unwrap().insert(k.clone(), v.clone());
            core::mem::forget(v);
            0
        }
        1 => {
            let m = _mem_db.read().unwrap();
            let v = m.get(&k).unwrap();
            forget!(v.clone())
        },
        3 => {
            _mem_db.read().unwrap().contains_key(&k) as u64
        },
        2 => {
            _mem_db.write().unwrap().remove(&k);
            0
        },
        _ => 0
    };
    core::mem::forget(k);
    ret
}