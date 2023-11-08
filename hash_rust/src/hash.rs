fn padding_bits(message: &mut Vec<u8>) -> Vec<u8> {
    let message_len_bits = (message.len() * 8) as u64;
    let padding_len_bits = (512 + 448 - (message_len_bits) % 512) % 512;
    let zero_padding_bytes = (padding_len_bits / 8) as usize;
    let mut zero_padding = vec![0u8; zero_padding_bytes - 1];

    let len_bytes: [u8; 8] = (message_len_bits).to_be_bytes();

    message.push(0x80);
    message.extend(zero_padding.iter());
    let len_bytes: [u8; 8] = (message_len_bits).to_be_bytes();
    for i in 0..len_bytes.len(){
    }
    message.extend(&len_bytes);

    message.clone()
}

pub const A: u32 = 0x6a09e667;
pub const B: u32 = 0xbb67ae85;
pub const C: u32 = 0x3c6ef372;
pub const D: u32 = 0xa54ff53a;
pub const E: u32 = 0x510e527f;
pub const F: u32 = 0x9b05688c;
pub const G: u32 = 0x1f83d9ab;
pub const H: u32 = 0x5be0cd19;

pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 
    0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 
    0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 
    0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 
    0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 
    0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 
    0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 
    0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 
    0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

pub fn initialize_w(input_message: &[u8]) -> [u32; 64] {
    let mut w: [u32; 64] = [0; 64];

    for i in 0..16 {
        let chunk = &input_message[i * 4..(i + 1) * 4];
        w[i] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        //println!("w{} = {:032b}", i, w[i]);
    }

    for i in 16..64 {
        let s0 = (w[i - 15].rotate_right(7)) ^ (w[i - 15].rotate_right(18)) ^ (w[i - 15] >> 3);
        let s1 = (w[i - 2].rotate_right(17)) ^ (w[i - 2].rotate_right(19)) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        //println!("w{} = {:032b}", i, w[i]);
    }
    
    w
}

fn ch(e: u32, f: u32, g: u32) -> u32 {
    (e & f) ^ (!e & g)
}

fn ma(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

fn sigma_a(a: u32) -> u32 {
    (a.rotate_right(2)) ^ (a.rotate_right(13)) ^ (a.rotate_right(22))
}

fn sigma_e(e: u32) -> u32 {
    (e.rotate_right(6)) ^ (e.rotate_right(11)) ^ (e.rotate_right(25))
}



pub fn compress_function(w: &mut [u32; 64]) -> [u32; 8] {
    let mut a = A;
    let mut b = B;
    let mut c = C;
    let mut d = D;
    let mut e = E;
    let mut f = F;
    let mut g = G;
    let mut h = H;
    let mut k = K;

    for i in 0..64 {
        let t1 = h.wrapping_add(sigma_e(e)).wrapping_add(ch(e, f, g)).wrapping_add(k[i]).wrapping_add(w[i]);
        let t2 = sigma_a(a).wrapping_add(ma(a, b, c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);

        //println!("a {:032b}, b {:032b}, c {:032b}, d {:032b}, e {:032b}, f {:032b}, g {:032b}, h {:032b}", a, b, c, d, e, f, g, h);

    }
    let mut h0: u32 = 0x6a09e667;
    let mut h1: u32 = 0xbb67ae85;
    let mut h2: u32 = 0x3c6ef372;
    let mut h3: u32 = 0xa54ff53a;
    let mut h4: u32 = 0x510e527f;
    let mut h5: u32 = 0x9b05688c;
    let mut h6: u32 = 0x1f83d9ab;
    let mut h7: u32 = 0x5be0cd19;

    h0 = h0.wrapping_add(a);
    h1 = h1.wrapping_add(b);
    h2 = h2.wrapping_add(c);
    h3 = h3.wrapping_add(d);
    h4 = h4.wrapping_add(e);
    h5 = h5.wrapping_add(f);
    h6 = h6.wrapping_add(g);
    h7 = h7.wrapping_add(h);


    [
         h0, h1, h2, h3, h4, h5, h6, h7,
    ]


}


fn u32_array_to_string(arr: &[u32; 8]) -> String {
    let mut result = String::new();
    for &value in arr.iter() {
        result.push_str(&format!("{:08x}", value));
    }
    result
}

pub fn own_hash(message: &mut Vec<u8>) -> String {
    let resized_message = padding_bits(message);
    let mut w = initialize_w(&resized_message);
    
    let hash = compress_function(&mut w);
    let hash_string = u32_array_to_string(&hash);
    hash_string
}
