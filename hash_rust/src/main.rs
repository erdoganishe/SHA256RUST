extern crate crypto_hash;
use crypto_hash::hex_digest;

mod hash;
use hash::{own_hash};

fn main() {
    let data = "Hello, world!";
    


    let mut message: Vec<u8> = b"Hello, World".to_vec(); 
    let sha_hash = hex_digest(crypto_hash::Algorithm::SHA256, &message);
    println!("SHA-256 Hash: {}", sha_hash);
    let hash = own_hash(&mut message.to_vec());
    println!("Own-256 Hash: {}", hash);
}