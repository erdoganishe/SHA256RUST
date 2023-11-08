extern crate crypto_hash;
use crypto_hash::hex_digest;

fn main() {
    let data = "Hello, world!";
    let hash = hex_digest(crypto_hash::Algorithm::SHA256, data.as_bytes());

    println!("SHA-256 Hash: {}", hash);
}