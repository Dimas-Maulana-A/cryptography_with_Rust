use std::io;

mod reverse_cipher;
mod caesar_cipher;
mod rot13_cipher;
mod rot18_cipher;
mod rot23_cipher;

fn main(){
    println!("select with number");
    println!("1. reverse cipher");
    println!("2. caesar cipher");
    println!("3. ROT13 cipher");
    println!("4. ROT18 cipher");
    println!("5. ROT23 cipher");
    // println!("6. Symentric cipher");
    // println!("7. Asymentric cipher");
    println!("6. exit");

    loop {
        println!("what are your options ?");
        let mut numbers = String::new();
        io::stdin().read_line(&mut numbers).unwrap();
        let int_numbers = numbers.trim().parse::<i32>().unwrap();

        if int_numbers == 1 {
            reverse_cipher::reverse_main();
        }else if  int_numbers == 2 {
            caesar_cipher::caesar_main();
        }else if  int_numbers == 3 {
            rot13_cipher::rot13_main();
        }else if int_numbers == 4{
            rot18_cipher::rot18_main();
        }else if int_numbers == 5{
            rot23_cipher::rot23_main();
        }else if int_numbers == 6{
            println!("Byee...");
            break;
        }
    }
    
}
