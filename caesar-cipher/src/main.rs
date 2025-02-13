use caesar_cipher::decrypt;
use caesar_cipher::encrypt;

fn main() {
    let plaintext = "Hello, World!";
    let shift = 3;
    let ciphertext = encrypt(plaintext, shift);
    println!("Ciphertext: {}", ciphertext);
    let decrypted_text = decrypt(&ciphertext, shift);
    println!("Decrypted text: {}", decrypted_text); 
}
