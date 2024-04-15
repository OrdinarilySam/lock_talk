mod aes;

pub fn aes_encrypt(input: Vec<u8>, key: Vec<u8>) {
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

    let num_blocks = input.len() / 16;
    for i in 0..num_blocks {
      let mut block: Vec<u8> = Vec::new();
      block.extend_from_slice(&input[i*16..(i+1)*16]);
      // for j in 0..16 {
      //   block.push(input[i*16 + j]);
      // }

      let mut encrypted = aes::cipher(block, num_rounds, &key_schedule);
      cipher_text.append(&mut encrypted);

    }

    let remainder = input.len() % 16;
    if remainder != 0 {
      let mut block: Vec<u8> = Vec::new();
      block.extend_from_slice(&input[num_blocks*16..num_blocks*16 + remainder]);
      // block.extend_from_slice(&[(16 - remainder) as usize; 16 - remainder]);

    }




}
