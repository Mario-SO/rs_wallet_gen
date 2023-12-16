use secp256k1::{rand::rngs::OsRng, PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};

pub struct EthWallet {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub address: Vec<u8>,
}

impl EthWallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        let public_key_serialized = public_key.serialize_uncompressed();

        let mut keccak = Keccak::v256();
        let mut hash = [0u8; 32];
        keccak.update(&public_key_serialized[1..]); // Skip the first byte
        keccak.finalize(&mut hash);

        let address = hash[12..].to_vec();

        EthWallet {
            secret_key,
            public_key,
            address,
        }
    }

    pub fn secret_key_hex(&self) -> String {
        self.secret_key.display_secret().to_string()
    }
}
