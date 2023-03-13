use std::io;

pub fn reverse_main(){
    println!("input your plan text : ");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();


    let cipher = reverse_cipher(&message);
    println!("reverse cipher : {}", cipher);
}

fn reverse_cipher(message: &str) -> String{
    let reversed = message.chars().rev().collect();
    return  reversed;
}