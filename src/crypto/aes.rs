mod galois;
mod sbox;

use galois::*;
use sbox::*;

use std::fmt::{Display, Error, Formatter};

struct ByteVec(Vec<u8>);

impl Display for ByteVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            let value = format!("{:02x}", num);
            comma_separated.push_str(&value);
            comma_separated.push_str(", ");
        }

        let value = format!("{:02x}", &self.0[self.0.len() - 1]);
        comma_separated.push_str(&value);
        write!(f, "[{}]", comma_separated)
    }
}

/* ----------- ENCRYPTION AND DECRYPTION ------------ */
fn cipher(input: Vec<u8>, num_rounds: u8, key_schedule: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut state = input;
    state = add_round_key(state, &key_schedule[0..4]);

    for round in 1..num_rounds as usize {
        state = sub_bytes(state);
        state = shift_rows(state);
        state = mix_columns(state);
        state = add_round_key(state, &key_schedule[round * 4..round * 4 + 4]);
    }
    // don't mix columns in final round
    state = sub_bytes(state);
    state = shift_rows(state);
    let num_rounds = num_rounds as usize;
    state = add_round_key(state, &key_schedule[num_rounds * 4..num_rounds * 4 + 4]);
    state
}

fn key_expansion(key: Vec<u8>) -> Vec<Vec<u8>> {
    let n_k: u8;
    let n_r: u8;
    match key.len() / 4 {
        4 => {
            n_k = 4;
            n_r = 10;
        }
        6 => {
            n_k = 6;
            n_r = 12;
        }
        8 => {
            n_k = 8;
            n_r = 14;
        }
        _ => panic!("Invalid key size!"),
    };

    let mut key_schedule: Vec<Vec<u8>> = Vec::new();

    for word in 0..n_k as usize {
        let mut temp = Vec::new();
        for byte in 0..4 {
            temp.push(key[word * 4 + byte]);
        }
        key_schedule.push(temp);
    }

    for i in n_k as usize..4 * (n_r + 1) as usize {
        let mut temp = key_schedule[i - 1].clone();
        if i as u8 % n_k == 0 {
            temp = sub_word(&rot_word(&temp[..])[..]);
            temp = gf_add_word(temp, r_con((i / n_k as usize) - 1));
        } else if n_k > 6 && i as u8 % n_k == 4 {
            temp = sub_word(&temp[..]);
        }
        key_schedule.push(gf_add_word(key_schedule[i - n_k as usize].clone(), temp))
    }

    key_schedule
}

/* ----------- ENCRYPTION FUNCTIONS ------------ */
fn add_round_key(mut state: Vec<u8>, round_key: &[Vec<u8>]) -> Vec<u8> {
    for column in 0..4 {
        for row in 0..4 {
            state[row + column * 4] ^= round_key[column][row];
        }
    }
    state
}

fn sub_bytes(mut state: Vec<u8>) -> Vec<u8> {
    for i in 0..state.len() {
        state[i] = s_box(state[i]);
    }
    state
}

fn shift_rows(mut state: Vec<u8>) -> Vec<u8> {
    // skip the first row
    for row in 1..4 {
        let mut new_row = Vec::new();
        for column in 0..4 {
            let new_column = (row + column) % 4;
            new_row.push(state[row + new_column * 4]);
        }
        for column in 0..4 {
            state[row + column * 4] = new_row[column];
        }
    }
    state
}

fn mix_columns(mut state: Vec<u8>) -> Vec<u8> {
    for column in 0..4 {
        let s0 = state[column * 4];
        let s1 = state[1 + column * 4];
        let s2 = state[2 + column * 4];
        let s3 = state[3 + column * 4];

        // multiply the column elements by a fixed matrix
        state[column * 4] = gf_mult(0x02, s0) ^ gf_mult(0x03, s1) ^ s2 ^ s3;
        state[1 + column * 4] = s0 ^ gf_mult(0x02, s1) ^ gf_mult(0x03, s2) ^ s3;
        state[2 + column * 4] = s0 ^ s1 ^ gf_mult(0x02, s2) ^ gf_mult(0x03, s3);
        state[3 + column * 4] = gf_mult(0x03, s0) ^ s1 ^ s2 ^ gf_mult(0x02, s3);
    }
    state
}

fn rot_word(word: &[u8]) -> Vec<u8> {
    let mut new_word: Vec<u8> = Vec::new();
    for i in 0..word.len() {
        new_word.push(word[(i + 1) % 4]);
    }
    new_word
}

/* ----------- DECRYPTION FUNCTIONS ------------ */

/* ----------- TESTING ------------ */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cipher_test_official() {
        let input: Vec<u8> = vec![
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];

        let key: Vec<u8> = vec![
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];

        let expected: Vec<u8> = vec![
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ];

        let key_schedule = key_expansion(key);
        let output = cipher(input, 10, &key_schedule);
        assert_eq!(expected, output);
    }

    #[test]
    fn key_expansion_128_test() {
        let key: Vec<u8> = vec![
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let expected: Vec<Vec<u8>> = vec![
            vec![0x2b, 0x7e, 0x15, 0x16],
            vec![0x28, 0xae, 0xd2, 0xa6],
            vec![0xab, 0xf7, 0x15, 0x88],
            vec![0x09, 0xcf, 0x4f, 0x3c],
            vec![0xa0, 0xfa, 0xfe, 0x17],
            vec![0x88, 0x54, 0x2c, 0xb1],
            vec![0x23, 0xa3, 0x39, 0x39],
            vec![0x2a, 0x6c, 0x76, 0x05],
            vec![0xf2, 0xc2, 0x95, 0xf2],
            vec![0x7a, 0x96, 0xb9, 0x43],
            vec![0x59, 0x35, 0x80, 0x7a],
            vec![0x73, 0x59, 0xf6, 0x7f],
            vec![0x3d, 0x80, 0x47, 0x7d],
            vec![0x47, 0x16, 0xfe, 0x3e],
            vec![0x1e, 0x23, 0x7e, 0x44],
            vec![0x6d, 0x7a, 0x88, 0x3b],
            vec![0xef, 0x44, 0xa5, 0x41],
            vec![0xa8, 0x52, 0x5b, 0x7f],
            vec![0xb6, 0x71, 0x25, 0x3b],
            vec![0xdb, 0x0b, 0xad, 0x00],
            vec![0xd4, 0xd1, 0xc6, 0xf8],
            vec![0x7c, 0x83, 0x9d, 0x87],
            vec![0xca, 0xf2, 0xb8, 0xbc],
            vec![0x11, 0xf9, 0x15, 0xbc],
            vec![0x6d, 0x88, 0xa3, 0x7a],
            vec![0x11, 0x0b, 0x3e, 0xfd],
            vec![0xdb, 0xf9, 0x86, 0x41],
            vec![0xca, 0x00, 0x93, 0xfd],
            vec![0x4e, 0x54, 0xf7, 0x0e],
            vec![0x5f, 0x5f, 0xc9, 0xf3],
            vec![0x84, 0xa6, 0x4f, 0xb2],
            vec![0x4e, 0xa6, 0xdc, 0x4f],
            vec![0xea, 0xd2, 0x73, 0x21],
            vec![0xb5, 0x8d, 0xba, 0xd2],
            vec![0x31, 0x2b, 0xf5, 0x60],
            vec![0x7f, 0x8d, 0x29, 0x2f],
            vec![0xac, 0x77, 0x66, 0xf3],
            vec![0x19, 0xfa, 0xdc, 0x21],
            vec![0x28, 0xd1, 0x29, 0x41],
            vec![0x57, 0x5c, 0x00, 0x6e],
            vec![0xd0, 0x14, 0xf9, 0xa8],
            vec![0xc9, 0xee, 0x25, 0x89],
            vec![0xe1, 0x3f, 0x0c, 0xc8],
            vec![0xb6, 0x63, 0x0c, 0xa6],
        ];

        assert_eq!(key_expansion(key), expected);
    }

    #[test]
    fn key_expansion_192_test() {
        let key: Vec<u8> = vec![
            0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90,
            0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b,
        ];
        let expected: Vec<Vec<u8>> = vec![
            vec![0x8e, 0x73, 0xb0, 0xf7],
            vec![0xda, 0x0e, 0x64, 0x52],
            vec![0xc8, 0x10, 0xf3, 0x2b],
            vec![0x80, 0x90, 0x79, 0xe5],
            vec![0x62, 0xf8, 0xea, 0xd2],
            vec![0x52, 0x2c, 0x6b, 0x7b],
            vec![0xfe, 0x0c, 0x91, 0xf7],
            vec![0x24, 0x02, 0xf5, 0xa5],
            vec![0xec, 0x12, 0x06, 0x8e],
            vec![0x6c, 0x82, 0x7f, 0x6b],
            vec![0x0e, 0x7a, 0x95, 0xb9],
            vec![0x5c, 0x56, 0xfe, 0xc2],
            vec![0x4d, 0xb7, 0xb4, 0xbd],
            vec![0x69, 0xb5, 0x41, 0x18],
            vec![0x85, 0xa7, 0x47, 0x96],
            vec![0xe9, 0x25, 0x38, 0xfd],
            vec![0xe7, 0x5f, 0xad, 0x44],
            vec![0xbb, 0x09, 0x53, 0x86],
            vec![0x48, 0x5a, 0xf0, 0x57],
            vec![0x21, 0xef, 0xb1, 0x4f],
            vec![0xa4, 0x48, 0xf6, 0xd9],
            vec![0x4d, 0x6d, 0xce, 0x24],
            vec![0xaa, 0x32, 0x63, 0x60],
            vec![0x11, 0x3b, 0x30, 0xe6],
            vec![0xa2, 0x5e, 0x7e, 0xd5],
            vec![0x83, 0xb1, 0xcf, 0x9a],
            vec![0x27, 0xf9, 0x39, 0x43],
            vec![0x6a, 0x94, 0xf7, 0x67],
            vec![0xc0, 0xa6, 0x94, 0x07],
            vec![0xd1, 0x9d, 0xa4, 0xe1],
            vec![0xec, 0x17, 0x86, 0xeb],
            vec![0x6f, 0xa6, 0x49, 0x71],
            vec![0x48, 0x5f, 0x70, 0x32],
            vec![0x22, 0xcb, 0x87, 0x55],
            vec![0xe2, 0x6d, 0x13, 0x52],
            vec![0x33, 0xf0, 0xb7, 0xb3],
            vec![0x40, 0xbe, 0xeb, 0x28],
            vec![0x2f, 0x18, 0xa2, 0x59],
            vec![0x67, 0x47, 0xd2, 0x6b],
            vec![0x45, 0x8c, 0x55, 0x3e],
            vec![0xa7, 0xe1, 0x46, 0x6c],
            vec![0x94, 0x11, 0xf1, 0xdf],
            vec![0x82, 0x1f, 0x75, 0x0a],
            vec![0xad, 0x07, 0xd7, 0x53],
            vec![0xca, 0x40, 0x05, 0x38],
            vec![0x8f, 0xcc, 0x50, 0x06],
            vec![0x28, 0x2d, 0x16, 0x6a],
            vec![0xbc, 0x3c, 0xe7, 0xb5],
            vec![0xe9, 0x8b, 0xa0, 0x6f],
            vec![0x44, 0x8c, 0x77, 0x3c],
            vec![0x8e, 0xcc, 0x72, 0x04],
            vec![0x01, 0x00, 0x22, 0x02],
        ];

        assert_eq!(key_expansion(key), expected);
    }

    #[test]
    fn key_expansion_256_test() {
        let key: Vec<u8> = vec![
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d,
            0x77, 0x81, 0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3,
            0x09, 0x14, 0xdf, 0xf4,
        ];
        let expected: Vec<Vec<u8>> = vec![
            vec![0x60, 0x3d, 0xeb, 0x10],
            vec![0x15, 0xca, 0x71, 0xbe],
            vec![0x2b, 0x73, 0xae, 0xf0],
            vec![0x85, 0x7d, 0x77, 0x81],
            vec![0x1f, 0x35, 0x2c, 0x07],
            vec![0x3b, 0x61, 0x08, 0xd7],
            vec![0x2d, 0x98, 0x10, 0xa3],
            vec![0x09, 0x14, 0xdf, 0xf4],
            vec![0x9b, 0xa3, 0x54, 0x11],
            vec![0x8e, 0x69, 0x25, 0xaf],
            vec![0xa5, 0x1a, 0x8b, 0x5f],
            vec![0x20, 0x67, 0xfc, 0xde],
            vec![0xa8, 0xb0, 0x9c, 0x1a],
            vec![0x93, 0xd1, 0x94, 0xcd],
            vec![0xbe, 0x49, 0x84, 0x6e],
            vec![0xb7, 0x5d, 0x5b, 0x9a],
            vec![0xd5, 0x9a, 0xec, 0xb8],
            vec![0x5b, 0xf3, 0xc9, 0x17],
            vec![0xfe, 0xe9, 0x42, 0x48],
            vec![0xde, 0x8e, 0xbe, 0x96],
            vec![0xb5, 0xa9, 0x32, 0x8a],
            vec![0x26, 0x78, 0xa6, 0x47],
            vec![0x98, 0x31, 0x22, 0x29],
            vec![0x2f, 0x6c, 0x79, 0xb3],
            vec![0x81, 0x2c, 0x81, 0xad],
            vec![0xda, 0xdf, 0x48, 0xba],
            vec![0x24, 0x36, 0x0a, 0xf2],
            vec![0xfa, 0xb8, 0xb4, 0x64],
            vec![0x98, 0xc5, 0xbf, 0xc9],
            vec![0xbe, 0xbd, 0x19, 0x8e],
            vec![0x26, 0x8c, 0x3b, 0xa7],
            vec![0x09, 0xe0, 0x42, 0x14],
            vec![0x68, 0x00, 0x7b, 0xac],
            vec![0xb2, 0xdf, 0x33, 0x16],
            vec![0x96, 0xe9, 0x39, 0xe4],
            vec![0x6c, 0x51, 0x8d, 0x80],
            vec![0xc8, 0x14, 0xe2, 0x04],
            vec![0x76, 0xa9, 0xfb, 0x8a],
            vec![0x50, 0x25, 0xc0, 0x2d],
            vec![0x59, 0xc5, 0x82, 0x39],
            vec![0xde, 0x13, 0x69, 0x67],
            vec![0x6c, 0xcc, 0x5a, 0x71],
            vec![0xfa, 0x25, 0x63, 0x95],
            vec![0x96, 0x74, 0xee, 0x15],
            vec![0x58, 0x86, 0xca, 0x5d],
            vec![0x2e, 0x2f, 0x31, 0xd7],
            vec![0x7e, 0x0a, 0xf1, 0xfa],
            vec![0x27, 0xcf, 0x73, 0xc3],
            vec![0x74, 0x9c, 0x47, 0xab],
            vec![0x18, 0x50, 0x1d, 0xda],
            vec![0xe2, 0x75, 0x7e, 0x4f],
            vec![0x74, 0x01, 0x90, 0x5a],
            vec![0xca, 0xfa, 0xaa, 0xe3],
            vec![0xe4, 0xd5, 0x9b, 0x34],
            vec![0x9a, 0xdf, 0x6a, 0xce],
            vec![0xbd, 0x10, 0x19, 0x0d],
            vec![0xfe, 0x48, 0x90, 0xd1],
            vec![0xe6, 0x18, 0x8d, 0x0b],
            vec![0x04, 0x6d, 0xf3, 0x44],
            vec![0x70, 0x6c, 0x63, 0x1e],
        ];

        assert_eq!(key_expansion(key), expected);
    }

    #[test]
    fn sub_bytes_test() {}
}