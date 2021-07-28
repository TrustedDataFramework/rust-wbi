// helper functions for tests
use crate::keys::PrivateSet;
use crate::member::Member;
use crate::mlsag::Mlsag;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::SeedableRng;

// There is an exact copy of this function in member.rs
// which is being used to generate nonces. The reason is because the code in this
// file should only be used for test
#[cfg(test)]
pub fn generate_rand_scalars(seed: u64, num: usize) -> Vec<Scalar> {
    let mut rng = rand::chacha::ChaChaRng::seed_from_u64(seed);
    let mut scalars = Vec::<Scalar>::with_capacity(num);

    for _ in 0..num {
        scalars.push(Scalar::random(&mut rng));
    }

    scalars
}

#[cfg(test)]
pub fn generate_private_set(seed: u64, num: usize) -> PrivateSet {
    let scalars = generate_rand_scalars(seed, num);
    PrivateSet(scalars)
}

#[cfg(test)]
pub fn generate_rand_points(num: usize) -> Vec<RistrettoPoint> {
    let mut rng = rand::chacha::ChaChaRng::seed_from_u64(199);
    let mut points = Vec::<RistrettoPoint>::with_capacity(num);

    for _ in 0..num {
        points.push(RistrettoPoint::random(&mut rng));
    }

    points
}

#[cfg(test)]
pub fn generate_decoy(num_keys: usize) -> Member {
    let points = generate_rand_points(num_keys);
    Member::new_decoy(28, points)
}

#[cfg(test)]
pub fn generate_decoys(num_decoys: usize, num_keys: usize) -> Vec<Member> {
    let mut decoys: Vec<Member> = Vec::with_capacity(num_decoys);
    for _ in 0..num_decoys {
        decoys.push(generate_decoy(num_keys));
    }
    decoys
}

#[cfg(test)]
pub fn generate_signer(num_keys: usize) -> Member {
    let scalars = generate_rand_scalars(197, num_keys);
    Member::new_signer(190, scalars)
}

#[cfg(test)]
pub fn generate_mlsag_with(num_decoys: usize, num_keys: usize) -> Mlsag {
    let mut mlsag = Mlsag::new();

    for _ in 0..num_decoys {
        mlsag.add_member(generate_decoy(num_keys));
    }

    mlsag
}
