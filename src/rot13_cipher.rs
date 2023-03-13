use std::io;

pub fn rot13_main() {
    println!("select options");
    println!("1. Encrypt ROT13");
    println!("2. Decrypt ROT13");

    println!("what are your options ?");
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let int_number = number.trim().parse::<i32>().unwrap();

    if int_number == 1 {
        println!("input your plain text : ");

        let mut plain = String::new();
        io::stdin().read_line(&mut plain).unwrap();
        rot13_encrypt(&plain);
    } else if int_number == 2 {
        println!("input your cipher text");

        let mut cipher = String::new();
        io::stdin().read_line(&mut cipher).unwrap();
        
        rot13_decrypt(&cipher);
    }
}

fn rot13_encrypt(input: &str) {
    let encrypted = rot13_cipher(input);
    println!("Encrypted: {}", encrypted);

}

fn rot13_decrypt(input: &str) {
    let decrypted = rot13_cipher(input);
    println!("Decrypt : {}", decrypted);
}

fn rot13_cipher(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + 13) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}
