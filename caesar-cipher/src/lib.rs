/*
This code defines two functions: encrypt and decrypt.
The encrypt function takes a plaintext string and a shift value, and returns the ciphertext string. The decrypt function takes a ciphertext string and a shift value,
and returns the plaintext string.

*/

pub fn encrypt(plaintext: &str, shift: u8) -> String {
    let mut ciphertext = String::new();
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = (c as u8 - base + shift) % 26 + base;
            ciphertext.push(shifted as char);
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}