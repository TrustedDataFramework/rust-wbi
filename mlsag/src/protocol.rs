extern crate curve25519_dalek;


#[cfg(test)]
mod test {
    #[test]
    fn test_protocol() {
        use crate::mlsag::Mlsag;
        use crate::tests_helper::generate_decoys;
        use crate::tests_helper::generate_signer;

        // Define setup parameters
        let num_keys = 2;
        let num_decoys = 11;
        let msg = b"hello world";
    
        // Define a mlsag object which will be used to create a signature
        let mut mlsag = Mlsag::new();
    
        // Generate and add decoys
        let decoys = generate_decoys(num_decoys, num_keys);
        for decoy in decoys {
            mlsag.add_member(decoy);
        }
    
        // Generate and add signer
        let signer = generate_signer(num_keys);
        mlsag.add_member(signer);
    
        let signature = mlsag.sign(msg).unwrap();
        let res = signature.verify(&mut mlsag.public_keys(), msg);
    
        assert!(res.is_ok())
    }
}

