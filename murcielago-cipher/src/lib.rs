use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref MURCIELAGO_MAP: HashMap<char, u32> = {
        let mut murcielago_map = HashMap::new();
        murcielago_map.insert('m', 0);
        murcielago_map.insert('u', 1);
        murcielago_map.insert('r', 2);
        murcielago_map.insert('c', 3);
        murcielago_map.insert('i', 4);
        murcielago_map.insert('e', 5);
        murcielago_map.insert('l', 6);
        murcielago_map.insert('a', 7);
        murcielago_map.insert('g', 8);
        murcielago_map.insert('o', 9);
        
        murcielago_map
    };

    static ref REVERSE_MURCIELAGO_MAP: HashMap<u32, char> = {
        MURCIELAGO_MAP.iter()
            .map(|(&key, &value)| (value, key))
            .collect()
    };
}

pub fn encrypt_murcielago(text: &str) -> String {

    let mut encrypted_text = String::new();

    for c in text.chars() {
        if let Some(&num) = MURCIELAGO_MAP.get(&c.to_ascii_lowercase()) {
            encrypted_text.push_str(&num.to_string());
        } else {
            encrypted_text.push(c);
        }
    }

    encrypted_text
}

pub fn decrypt_murcielago(text: &str) -> String {

    let mut decrypted_text = String::new();

    for c in text.chars() {
        if let Some(digit) = c.to_digit(10) {
            if let Some(&letter) = REVERSE_MURCIELAGO_MAP.get(&digit) {
                decrypted_text.push(letter);
            } else {
                decrypted_text.push(c);
            }
        } else {
            decrypted_text.push(c);
        }
    }

    decrypted_text
}