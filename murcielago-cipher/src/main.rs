use murcielago_cipher::{decrypt_murcielago, encrypt_murcielago};

fn main() {
    let text = "amigo";
    let encrypted_text = encrypt_murcielago(text);
    println!("MURCIÉLAGO Cipher: {}", encrypted_text);
    println!("Original text: {}", decrypt_murcielago(&encrypted_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_murcielago() {
        assert_eq!(encrypt_murcielago("AMIGO"), "70489");
        assert_eq!(encrypt_murcielago("murcielago"), "0123456789");
        assert_eq!(encrypt_murcielago("HELLO"), "H5669"); 
    }

    #[test]
    fn test_decrypt_murcielago() {
        assert_eq!(decrypt_murcielago("70489"), "amigo");
        assert_eq!(decrypt_murcielago("0123456789"), "murcielago");
        assert_eq!(decrypt_murcielago("H5669"), "Hello");
    }
}
