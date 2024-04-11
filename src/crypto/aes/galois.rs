pub fn gf_add_word(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
  if left.len() != right.len() {
    panic!("Trying to add words with unequal lengths!");
  }

  let mut new_word: Vec<u8> = Vec::new();
  for i in 0..left.len() {
    new_word.push(left[i] ^ right[i]);
  }
  new_word
}

pub const RCON: [[u8; 4]; 10] = [
  [0x01, 0, 0, 0],
  [0x02, 0, 0, 0],
  [0x04, 0, 0, 0],
  [0x08, 0, 0, 0],
  [0x10, 0, 0, 0],
  [0x20, 0, 0, 0],
  [0x40, 0, 0, 0],
  [0x80, 0, 0, 0],
  [0x1b, 0, 0, 0],
  [0x36, 0, 0, 0]
];

pub fn r_con(index: usize) -> Vec<u8> {
  Vec::from(RCON[index])
}

pub fn gf_mult(mut a: u8, mut b: u8) -> u8 {
    let mut result: u8 = 0;
    let irreducible: u8 = 0b00011011;

    for _ in 0..8 {
      if b & 1 == 1 {
        result ^= a;
      }

      // xor with irreducible if there is a carry
      let carry = a & 0x80;
      a <<= 1;
      if carry != 0 {
        a ^= irreducible;
      }
      b >>= 1;
    }

    result
}