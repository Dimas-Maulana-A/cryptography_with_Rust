use std::io;

pub fn rot23_main() {
    println!("select options");
    println!("1. Encrypt ROT23");
    println!("2. Decrypt ROT23");

    println!("what are your options ?");
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let int_number = number.trim().parse::<i32>().unwrap();

    if int_number == 1 {
        println!("input your plain text : ");

        let mut plain = String::new();
        io::stdin().read_line(&mut plain).unwrap();
        rot23_encrypt(&plain);
    } else if int_number == 2 {
        println!("input your cipher text");

        let mut cipher = String::new();
        io::stdin().read_line(&mut cipher).unwrap();
        
        rot23_decrypt(&cipher);
    }
}

fn rot23_encrypt(input: &str) {
    let encrypted = rot23_cipher(input);
    println!("Encrypted: {}", encrypted);

}

fn rot23_decrypt(input: &str) {
    let decrypted = rot23_plain(input);
    println!("Decrypt : {}", decrypted);
}

fn rot23_cipher(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars(){
        match  c {
            'A'..= 'Z' => result.push((b'A' + (c as u8 - b'A' + 23) % 26)as char),
            'a'..= 'z' => result.push((b'a' + (c as u8 - b'a' + 23) % 26)as char),
            _ => result.push(c),
        }
    }
    result
}

fn rot23_plain(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars(){
        match  c {
            'A'..= 'Z' => result.push((b'A' + (c as u8 - b'A' + 3) % 26)as char),
            'a'..= 'z' => result.push((b'a' + (c as u8 - b'a' + 3) % 26)as char),
            _ => result.push(c),
        }
    }
    result
}
