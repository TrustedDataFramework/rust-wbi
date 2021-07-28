extern crate alloc;
extern crate core;
extern crate jni;
extern crate libsm;
extern crate mlsag;
extern crate num_bigint;

use std::ptr::null_mut;

use alloc::vec::*;
use jni::JNIEnv;
use libsm::sm2;
use libsm::sm2::signature::Signature;
use libsm::sm3::hash;
use mlsag::member::Member;
use num_bigint::BigUint;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::{GlobalRef, JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jarray, jboolean, jbyteArray, jint, jlong, jobjectArray, jstring};

const RT_EX: &'static str = "java/lang/RuntimeException";

#[no_mangle]
pub extern "system" fn Java_org_tdf_natives_Crypto_sm3(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    input: jbyteArray,
) -> jbyteArray {
    match sm3(env, input) {
        Ok(o) => o,
        Err(e) => {
            env.throw_new(RT_EX, e.0);
            null_mut()
        }
    }
}

fn sm3(env: JNIEnv, input: jbyteArray) -> Result<jbyteArray, ChainErr> {
    let _input = env.convert_byte_array(input)?;
    let r = hash::Sm3Hash::new(&_input).get_hash();
    let output = env.byte_array_from_slice(&r)?;
    Ok(output)
}

#[no_mangle]
pub extern "system" fn Java_org_tdf_natives_Crypto_sm2PkFromSk(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _priv: jbyteArray,
    _compress: jboolean,
) -> jbyteArray {
    match sm2_pk_from_sk(env, _priv, _compress) {
        Ok(o) => o,
        Err(e) => {
            env.throw_new(RT_EX, e.0);
            null_mut()
        }
    }
}

fn sm2_pk_from_sk(
    env: JNIEnv,
    _priv: jbyteArray,
    _compress: jboolean,
) -> Result<jbyteArray, ChainErr> {
    let private_key = env.convert_byte_array(_priv)?;
    let compress = _compress != 0;
    let sig_ctx = sm2::signature::SigCtx::new();
    let ecc_ctx = sm2::ecc::EccCtx::new();
    let sk = BigUint::from_bytes_be(&private_key);
    let p = sig_ctx.pk_from_sk_checked(&sk)?;
    Ok(env.byte_array_from_slice(&ecc_ctx.point_to_bytes(&p, compress))?)
}

#[no_mangle]
pub extern "system" fn Java_org_tdf_natives_Crypto_sm2Verify(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _seed: jlong,
    _message: jbyteArray,
    _pub_key: jbyteArray,
    _sig: jbyteArray,
) -> jboolean {
    match sm2_verify(env, _seed, _message, _pub_key, _sig) {
        Ok(o) => o,
        Err(e) => {
            env.throw_new(RT_EX, e.0);
            0
        }
    }
}

struct ChainErr(pub &'static str);

impl From<jni::errors::Error> for ChainErr {
    fn from(_: jni::errors::Error) -> Self {
        ChainErr("jni error")
    }
}

impl From<sm2::error::Sm2Error> for ChainErr {
    fn from(_: sm2::error::Sm2Error) -> Self {
        ChainErr("sm2 error")
    }
}

fn sm2_verify(
    env: JNIEnv,
    _seed: jlong,
    _message: jbyteArray,
    _pub_key: jbyteArray,
    _sig: jbyteArray,
) -> Result<jboolean, ChainErr> {
    libsm::seed(_seed as u64);
    let sig = env.convert_byte_array(_sig)?;

    if sig.len() != 64 {
        return Err(ChainErr("invalid signatrue size"));
    }

    let sig_ctx = sm2::signature::SigCtx::new();
    let s: Signature = Signature::new(&sig[..32], &sig[32..]);
    let ecc_ctx = sm2::ecc::EccCtx::new();
    let decoded = env.convert_byte_array(_pub_key)?;
    let message = env.convert_byte_array(_message)?;
    let pk = ecc_ctx.bytes_to_point(&decoded)?;

    if sig_ctx.verify(&message, &pk, &s) {
        Ok(1)
    } else {
        Ok(0)
    }
}

#[no_mangle]
pub extern "system" fn Java_org_tdf_natives_Crypto_sm2Sign(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _seed: jlong,
    _priv: jbyteArray,
    _msg: jbyteArray,
) -> jbyteArray {
    match sm2_sign(env, _seed, _priv, _msg) {
        Ok(o) => o,
        Err(e) => {
            env.throw_new(RT_EX, e.0);
            null_mut()
        }
    }
}

fn sm2_sign(
    env: JNIEnv,
    _seed: jlong,
    _priv: jbyteArray,
    _msg: jbyteArray,
) -> Result<jbyteArray, ChainErr> {
    libsm::seed(_seed as u64);
    let private_key = env.convert_byte_array(_priv)?;
    let message = env.convert_byte_array(_msg)?;
    let c = sm2::signature::SigCtx::new();
    let sk = BigUint::from_bytes_be(&private_key);
    let sig = c.sign(&message, &sk, &c.pk_from_sk_checked(&sk)?);
    let mut r = [0; 32];
    let mut s = [0; 32];
    let _r = sig.get_r().to_bytes_be();
    r[32 - _r.len()..].copy_from_slice(&_r);
    let _s = sig.get_s().to_bytes_be();
    s[32 - _s.len()..].copy_from_slice(&_s);
    let mut ret = Vec::with_capacity(64);
    ret.extend_from_slice(&r);
    ret.extend_from_slice(&s);
    Ok(env.byte_array_from_slice(&ret)?)
}

#[no_mangle]
pub extern "system" fn Java_org_tdf_natives_Crypto_mlsagGetSk(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _seed: jlong,
) -> jbyteArray {
    let sk = mlsag::mlsag::random_sk(_seed as u64);
    match env.byte_array_from_slice(&sk) {
        Ok(o) => o,
        Err(_) => {
            env.throw_new(RT_EX, "jni error");
            null_mut()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_tdf_natives_Crypto_mlsagPkFromSk(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _privateKey: jbyteArray,
    _compress: jboolean
) -> jbyteArray {
    null_mut()
}

#[no_mangle]
pub extern "system" fn Java_org_tdf_natives_Crypto_mlsagSign(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _seed: jlong,
    _privateKey: jobjectArray,
    _compress: jboolean
) -> jbyteArray {
    null_mut()
}

fn mlsag_sign(env: JNIEnv, _seed: jlong, _privateKey: jobjectArray, msg: jbyteArray) -> Result<jbyteArray, ChainErr> {
    let cnt = env.get_array_length(_privateKey)? as usize;
    let mut v: Vec<Vec<u8>> = Vec::with_capacity(cnt);

    for i in 0..cnt {
        let _priv = env.get_object_array_element(_privateKey, i as i32)?;
        let _priv_bytes = env.convert_byte_array(_priv.into_inner())?;
        v.push(_priv_bytes)
    }

    let signer = Member::new_signer_from_bytes(_seed as u64, v);
    let mut mlsag = mlsag::mlsag::Mlsag::new();


    Ok(null_mut())
}