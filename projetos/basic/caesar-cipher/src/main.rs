use std::io::{self, Write};

fn shift_character(character: &mut char, start: char, end: char, shift: i64) {
    let start: u32 = start as u32;
    let end: u32 = end as u32;
    let char_value: u32 = *character as u32;
    let interval: u32 = end - start + 1;
    let shift: i64 = shift % interval as i64;
    let mut shifting: i64 = ((char_value as i64 - start as i64 + shift) % interval as i64) + start as i64;

    let is_shift_negative: bool = shift < 0;
    let is_shift_wrapping: bool = (shift * -1) > (char_value as i64 - start as i64);

    if is_shift_negative && is_shift_wrapping {
        shifting += interval as i64;
    }

    let shifted: u32 = shifting as u32;

    *character = char::from_u32(shifted).unwrap();
}

fn encrypt_text(text: &str, shift: i64) -> String {
    let mut chars: Vec<char> = text.chars().collect();

    for character in &mut chars {
        let start: char;
        let end: char;

        match character {
            'A'..='Z' => {
                start = 'A';
                end = 'Z';
            },
            'a'..='z' => {
                start = 'a';
                end = 'z';
            },
            _ => continue
        };

        shift_character(character, start, end, shift);
    }
    
    chars.into_iter().collect()
}

fn encrypt_operation(text: &str, key: u32) -> String {
    encrypt_text(text, key as i64)
}

fn decrypt_operation(text: &str, key: u32) -> String {
    encrypt_text(text, key as i64 * -1)
}

fn main() {
    println!("--- Caesar Cipher ---\n");

    loop {
        let encryption_operations = ['e', 'd'];

        let mut operation = String::new();
        let mut text = String::new();
        let mut key = String::new();

        println!(" ~  Operations");
        println!("[e] Encrypt text");
        println!("[d] Decrypt text");
        println!("[q] Quit");
        println!();
        io::stdout().flush().unwrap();
        
        print!("Operation: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut operation).expect("Input error.");

        let operation: char = operation
            .trim()
            .chars()
            .collect::<Vec<char>>()[0]
            .to_lowercase()
            .next()
            .unwrap();

        if !encryption_operations.contains(&operation) {
            if operation == 'q' {
                break;
            }

            println!("\nInvalid operation.\n");
            continue;
        }

        print!("Text: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut text).expect("Text input error.");

        print!("Key: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut key).expect("Key input error.");

        let key: u32 = match key.trim().parse() {
            Ok(key) => key,
            Err(_) => {
                println!("\nInvalid key.\n");
                continue;
            }
        };
        
        match operation {
            'e' => println!("\nEncrypted text: {}", encrypt_operation(text.as_str(), key)),
            'd' => println!("\nDecrypted text: {}", decrypt_operation(text.as_str(), key)),
            'q' => break,
            _ => ()
        };
    }
}