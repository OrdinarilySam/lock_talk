use super::util::*;

/* ----------- DECRYPTION FUNCTIONS ------------ */
pub fn inv_shift_rows(mut state: Vec<u8>) -> Vec<u8> {
    for row in 1..4 {
        let mut new_row = Vec::new();
        for column in 0..4 {
            let new_column = (4 + column - row) % 4;
            new_row.push(state[row + new_column * 4]);
        }
        for column in 0..4 {
            state[row + column * 4] = new_row[column];
        }
    }
    state
}

pub fn inv_sub_bytes(mut state: Vec<u8>) -> Vec<u8> {
    for i in 0..state.len() {
        state[i] = inv_s_box(state[i]);
    }
    state
}

pub fn inv_mix_columns(mut state: Vec<u8>) -> Vec<u8> {
    for column in 0..4 {
        let s0 = state[column * 4];
        let s1 = state[column * 4 + 1];
        let s2 = state[column * 4 + 2];
        let s3 = state[column * 4 + 3];

        // multiply the column elements by a fixed matrix
        state[column * 4] =
            gf_mult(0x0e, s0) ^ gf_mult(0x0b, s1) ^ gf_mult(0x0d, s2) ^ gf_mult(0x09, s3);
        state[column * 4 + 1] =
            gf_mult(0x09, s0) ^ gf_mult(0x0e, s1) ^ gf_mult(0x0b, s2) ^ gf_mult(0x0d, s3);
        state[column * 4 + 2] =
            gf_mult(0x0d, s0) ^ gf_mult(0x09, s1) ^ gf_mult(0x0e, s2) ^ gf_mult(0x0b, s3);
        state[column * 4 + 3] =
            gf_mult(0x0b, s0) ^ gf_mult(0x0d, s1) ^ gf_mult(0x09, s2) ^ gf_mult(0x0e, s3);
    }
    state
}