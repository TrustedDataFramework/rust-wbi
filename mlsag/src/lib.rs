#[macro_use]
extern crate core;
#[macro_use]
extern crate alloc;
extern crate curve25519_dalek;

pub mod constants;
pub mod keys;
pub mod member;
pub mod mlsag;
pub mod signature;
pub mod tests_helper;

mod protocol;
mod transcript;

use constants::BASEPOINT;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use rand::SeedableRng;
use curve25519_dalek::scalar::Scalar;
use signature::Signature;


pub fn generate_decoys(seed: u64, count: usize) -> Vec<Word> {
    let mut rng = rand::chacha::ChaChaRng::seed_from_u64(seed);
    let mut points = Vec::<[u8; 32]>::with_capacity(count);

    for _ in 0..count {
        points.push(RistrettoPoint::random(&mut rng).compress().0);
    }
    points
}

pub fn generate_signer(seed: u64) -> [u8; 32] {
    let mut rng = rand::chacha::ChaChaRng::seed_from_u64(seed);
    Scalar::random(&mut rng).to_bytes()
}

pub fn pk_from_sk(sk: [u8; 32]) -> [u8; 32] {
    let s = Scalar::from_bits(sk);
    let pk = s * BASEPOINT;
    pk.compress().0
}

use crate::member::Member;
use crate::mlsag::Mlsag;

pub type Word = [u8; 32];

pub fn sign(seed: u64, signer: &Word, decoys: &[Word], msg: &[u8]) -> (Word, Vec<Word>, Vec<Word> ) {
    let mut mlsag = Mlsag::new();

    let points: Vec<RistrettoPoint> = decoys.iter().map(|x| CompressedRistretto(x.clone()).decompress().unwrap()).collect();
    
    for p in &points {
        mlsag.add_member(Member::new_decoy(seed, vec![p.clone()]))
    }


    // add signer
    mlsag.add_member(
        Member::new_signer(
            seed + decoys.len() as u64, 
            vec![Scalar::from_bits(signer.clone())]
        )
    );

    let signature = mlsag.sign(msg).unwrap();
    (
        signature.challenge.to_bytes(), 
        signature.responses.iter().map(|x| x.to_bytes()).collect(), 
        signature.key_images.iter().map(|x| x.0).collect()
    )
}

pub fn verify(seed: u64, msg: &[u8], decoys: &[Word], challenge: &Word, responses:  &[Word], key_images: &[Word]) -> bool {
    let sig = Signature {
        challenge: Scalar::from_bits(challenge.clone()),
        responses: responses.iter().map(|x| Scalar::from_bits(x.clone())).collect(),
        key_images: key_images.iter().map(|x| CompressedRistretto(x.clone())).collect(),
    };

    let mut mlsag = Mlsag::new();
    let points: Vec<RistrettoPoint> = decoys.iter().map(|x| CompressedRistretto(x.clone()).decompress().unwrap()).collect();
    for p in points {
        mlsag.add_member(Member::new_decoy(seed, vec![p]));
    }
    match sig.verify(&mut mlsag.public_keys(), msg) {
        Ok(_) => true,
        _ => false
    }
}


#[cfg(test)]
mod test {
    use crate::member::Member;
    use crate::mlsag::Mlsag;
    use crate::*;

    #[test]
    fn test() {
        let msg = b"hello world";
        let mut mlsag = Mlsag::new();
        let mut decoys = generate_decoys(5, 5);
        let signer = generate_signer(6);
        let signer_pk = CompressedRistretto(pk_from_sk(signer)).decompress().unwrap();

        let signature = sign(8, &signer, &decoys, msg);

        decoys.push(signer_pk.compress().0);
        assert!(verify(8, msg, &decoys, &signature.0, &signature.1, &signature.2));
    }
}