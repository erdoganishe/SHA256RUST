extern crate crypto_hash;
use crypto_hash::hex_digest;

mod hash;
use hash::{own_hash};

fn main() {
    let data = "Hello, world!";
    let hash = hex_digest(crypto_hash::Algorithm::SHA256, data.as_bytes());

    println!("SHA-256 Hash: {}", hash);

    let mut message: Vec<u8> = b"Hello, World".to_vec(); 
    let hash = own_hash(&mut message);
    println!("Own-256 Hash: {}", hash);
}