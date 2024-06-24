mod aes;

pub fn aes_encrypt(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let num_rounds: u8 = match key.len() / 4 {
        4 => 10,
        6 => 12,
        8 => 14,
        _ => panic!("Invalid key length"),
    };

    let key_schedule = aes::key_expansion(key);

    //ECB
    // take the blocks
    let mut cipher_text: Vec<u8> = Vec::new();

    // Encrypt the even sized blocks
    let num_blocks = input.len() / 16;
    for i in 0..num_blocks {
        let mut block: Vec<u8> = Vec::new();
        block.extend_from_slice(&input[i * 16..(i + 1) * 16]);

        let mut encrypted = aes::cipher(block, num_rounds, &key_schedule);
        cipher_text.append(&mut encrypted);
    }

    // Encrypt the remainder
    let remainder = input.len() % 16;
    let padding = 16 - remainder;

    // if there is no remainder, add a full block of padding
    if remainder == 0 {
        let block = vec![padding as u8; 16];
        let mut encrypted = aes::cipher(block, num_rounds, &key_schedule);
        cipher_text.append(&mut encrypted);
    }
    // otherwise, add whatever padding is left
    else {
        let mut block: Vec<u8> = Vec::new();
        block.extend_from_slice(&input[num_blocks * 16..num_blocks * 16 + remainder]);
        block.resize(16, padding as u8);

        let mut encrypted = aes::cipher(block, num_rounds, &key_schedule);
        cipher_text.append(&mut encrypted);
    }

    cipher_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_128bit_1pad() {
        let key: Vec<u8> = vec![b'A'; 16];
        let input: Vec<u8> = vec![b'A'; 15];

        let result = aes_encrypt(input, key);

        let expected: Vec<u8> = vec![
            0xC5, 0x5B, 0xAD, 0xE2,
            0xF2, 0xB3, 0x26, 0x4D,
            0x4C, 0xA2, 0x8E, 0x59,
            0xE9, 0x81, 0x38, 0x8A,
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_128bit_block_pad() {
        let key: Vec<u8> = vec![b'A'; 16];
        let input: Vec<u8> = vec![b'A'; 16];

        let result = aes_encrypt(input, key);

        let expected: Vec<u8> = vec![
            0xF8, 0xCB, 0xA1, 0xAA,
            0x5B, 0x51, 0x20, 0xB4, 
            0xF2, 0xFD, 0xDA, 0x1B, 
            0x26, 0xCA, 0x01, 0x58,
            0x05, 0x15, 0xBE, 0x1D,
            0x9A, 0xFB, 0x4C, 0x54, 
            0xA2, 0x03, 0x90, 0x97, 
            0x3F, 0x58, 0x28, 0xD8, 
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_192bit_1pad() {
        let key: Vec<u8> = vec![b'A'; 24];
        let input: Vec<u8> = vec![b'A'; 15];

        let result = aes_encrypt(input, key);

        let expected: Vec<u8> = vec![
            0xE6, 0xA4, 0xA5, 0x9B, 
            0xF8, 0xF9, 0x60, 0x3B, 
            0x21, 0x2E, 0x51, 0x53, 
            0x2B, 0x7E, 0xD7, 0x9B, 
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_192bit_block_pad() {
        let key: Vec<u8> = vec![b'A'; 24];
        let input: Vec<u8> = vec![b'A'; 16];

        let result = aes_encrypt(input, key);

        let expected: Vec<u8> = vec![
            0xF4, 0xE1, 0x11, 0x8C, 
            0x9C, 0xCC, 0x60, 0x27, 
            0x92, 0x90, 0x62, 0xED, 
            0xC4, 0xDC, 0x5E, 0xD0, 
            0x07, 0x95, 0xC8, 0x8E, 
            0xC1, 0xF2, 0x01, 0x44, 
            0x72, 0x69, 0x5B, 0x0E, 
            0x65, 0xF6, 0x93, 0x66, 
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_256bit_1pad() {
        let key: Vec<u8> = vec![b'A'; 32];
        let input: Vec<u8> = vec![b'A'; 15];

        let result = aes_encrypt(input, key);

        let expected: Vec<u8> = vec![
            0xE3, 0xA7, 0x30, 0x6E, 
            0x06, 0x96, 0x1D, 0x7B, 
            0x59, 0x0C, 0xBB, 0x67, 
            0x9B, 0x6A, 0x8A, 0x8A, 
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_256bit_block_pad() {
        let key: Vec<u8> = vec![b'A'; 32];
        let input: Vec<u8> = vec![b'A'; 16];

        let result = aes_encrypt(input, key);

        let expected: Vec<u8> = vec![
            0x20, 0x7C, 0xA0, 0xEE, 
            0x7F, 0x5B, 0xDB, 0x88, 
            0x97, 0xCA, 0xA7, 0xB1, 
            0xF8, 0xFF, 0x21, 0x57, 
            0x98, 0xCC, 0x2F, 0x3F, 
            0x47, 0x6B, 0x40, 0xB7, 
            0xEA, 0x03, 0x2F, 0x33, 
            0xAF, 0x02, 0x3B, 0x61, 
        ];

        assert_eq!(result, expected);
    }

}
