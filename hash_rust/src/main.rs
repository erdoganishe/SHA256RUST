extern crate crypto_hash;
use crypto_hash::hex_digest;

mod hash;
use hash::{own_hash};
use std::time::{Instant};

fn main() {
    let mut message: Vec<u8> = b"Hello, World".to_vec();

    let start = Instant::now();
    let hash = own_hash(&mut message.to_vec());
    let duration = start.elapsed();

    println!("Own-256 Hash: {:?}", hash);
    println!("Own-256 Execution Time: {:?}", duration);

    let start = Instant::now();
    let sha_hash = hex_digest(crypto_hash::Algorithm::SHA256, &message);
    let duration = start.elapsed();
    println!("SHA-256 Hash: {:?}", sha_hash);
    println!("SHA-256 Execution Time: {:?}", duration);
}
