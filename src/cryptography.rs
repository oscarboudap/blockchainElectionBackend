use ring::signature::{Ed25519KeyPair, KeyPair, Signature};
use rand::rngs::OsRng;

// Generate public-private key pairs
pub fn generate_keys() -> Ed25519KeyPair {
    let rng = OsRng;
    let key_pair = Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate key pair");
    Ed25519KeyPair::from_pkcs8(key_pair.as_ref()).expect("Failed to parse key pair")
}

// Sign voting data with the voter's private key
pub fn sign_vote(vote_data: &[u8], key_pair: &Ed25519KeyPair) -> Signature {
    key_pair.sign(vote_data)
}

// Verify vote signature with the voter's public key
pub fn verify_vote_signature(vote_data: &[u8], signature: &Signature, public_key: &[u8]) -> bool {
    let peer_public_key = ring::signature::UnparsedPublicKey::new(
        &ring::signature::ED25519, 
        public_key
    );
    peer_public_key.verify(vote_data, signature.as_ref()).is_ok()
}
