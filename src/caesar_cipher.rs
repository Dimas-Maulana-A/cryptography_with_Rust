use std::io;

pub fn caesar_main() {
    let mut message: String = String::new();
    let mut key: String = String::new();

    println!("input message :");
    io::stdin().read_line(&mut message).unwrap();

    println!("input key :");
    io::stdin().read_line(&mut key).unwrap();
    let i8_key = key.trim().parse::<i8>().unwrap();

    if i8_key > 0 {
        let u8_key: u8 = key.trim().parse::<u8>().unwrap();
        let ciphered: String = caesar_cipher_encrypt(&message, u8_key);
        println!("caesar cipher text : {}", ciphered);
    } else if i8_key < 0 {
        let u8_key: u8 = i8_key.abs().try_into().unwrap();
        caesar_cipher_decrypt(&message, u8_key);
    }
}

fn caesar_cipher_encrypt(message: &str, key: u8) -> String {
    let mut ciphered = String::new();

    for c in message.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = (c as u8).wrapping_sub(base);
            let encoded = (offset + key) % 26 + base;
            ciphered.push(encoded as char);
        } else {
            ciphered.push(c)
        }
    }
    return ciphered;
}

fn caesar_cipher_decrypt(message: &str, key: u8) {
    let mut ikey = key;
    while ikey > 0 {
        let plained:String = caesar_cipher_encrypt(&message, 26 - ikey);
        println!("caesar plain text [-{}] : {}", ikey, plained);
        ikey -= 1;
    }
}
