pub mod crypto;

use rand::Rng;
use hex;

pub fn key_gen(size: u16) -> Vec<u8>{
    if ![128, 192, 256].contains(&size) {
        panic!("Invalid key size")
    }
    let mut key = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0..size / 8 {
        key.push(rng.gen());
    }

    key
}

pub fn encrypt(plaintext: String) -> (String, String) {
    let key = key_gen(256);

    let ciphertext = crypto::aes_encrypt(plaintext.as_bytes().to_vec().clone(), key.clone());

    return (hex::encode(ciphertext), hex::encode(key));
}

pub fn decrypt(ciphertext: String, key: String) -> String {
    let plaintext = crypto::aes_decrypt(hex::decode(ciphertext).unwrap(), hex::decode(key).unwrap());

    return String::from_utf8(plaintext).unwrap();
}
