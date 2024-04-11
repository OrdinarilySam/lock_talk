use super::util::*;

/* ----------- ENCRYPTION FUNCTIONS ------------ */
pub fn sub_bytes(mut state: Vec<u8>) -> Vec<u8> {
    for i in 0..state.len() {
        state[i] = s_box(state[i]);
    }
    state
}

pub fn shift_rows(mut state: Vec<u8>) -> Vec<u8> {
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

pub fn mix_columns(mut state: Vec<u8>) -> Vec<u8> {
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

