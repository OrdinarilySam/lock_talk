use std::io::{self, Write};

fn main(){
    print!("Encrypt or Decrypt: (e/d): ");
    io::stdout().flush().expect("Failed to flush stdout");


    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("failed to read line");

    let trimmed_selection = selection.trim();

    if trimmed_selection.eq_ignore_ascii_case("e") {
        print!("Enter value to encrypt: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut plaintext = String::new();

        io::stdin()
            .read_line(&mut plaintext)
            .expect("failed to read line");

        let trimmed_plaintext = plaintext.trim();

        let (ciphertext, key) = lock_talk::encrypt(trimmed_plaintext.to_string());

        println!("Ciphertext: {}\nKey: {}", ciphertext, key);

    } else if trimmed_selection.eq_ignore_ascii_case("d") {
        print!("Enter value to decrypt: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut ciphertext = String::new();

        io::stdin()
            .read_line(&mut ciphertext)
            .expect("failed to read line");

        let trimmed_ciphertext = ciphertext.trim();

        print!("Enter key: ");

        let mut key = String::new();

        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut key)
            .expect("failed to read line");

        let trimmed_key = key.trim();

        let plaintext = lock_talk::decrypt(
            trimmed_ciphertext.to_string(), 
            trimmed_key.to_string());

        println!("Decrypted: {}", plaintext);

    }
    
    
}

