use std::io;
use std::io::Write;

fn shift_forward(c: u8, start: u8, end: u8, shift: u32) -> u8 {
    let interval = (end - start + 1) as u32;

    (start as u32 + ((c as u32 + shift) - start as u32) % interval) as u8
}

fn shift_backwards(c: u8, start: u8, end: u8, shift: u32) -> u8 {
    let interval = (end - start + 1) as u32;

    (start as u32 + ((c as u32 + interval - (shift % interval) - start as u32) % interval)) as u8
}

fn transform(text: &str, key: u32, operation: char) -> String {
    let mut bytes = text.as_bytes().to_vec();

    for c in &mut bytes {
        let start: u8;
        let end: u8;

        match c {
            b'A'..=b'Z' => {
                start = b'A';
                end = b'Z';
            },
            b'a'..=b'z' => {
                start = b'a';
                end = b'z';
            },
            _ => continue
        };

        *c = match operation {
            'e' => shift_forward(*c, start, end, key),
            'd' => shift_backwards(*c, start, end, key),
            _ => *c
        };
    }
    
    String::from_utf8(bytes).expect("Error.")
}

fn encode(text: &str, key: u32) -> String {
    transform(text, key, 'e')
}

fn decode(text: &str, key: u32) -> String {
    transform(text, key, 'd')
}

fn caesar_cipher() {
    loop {
        let transform_ops = ['e', 'd'];

        let mut operation = String::new();
        let mut text = String::new();
        let mut key = String::new();

        println!(" ~  Operations");
        println!("[e] Encode text");
        println!("[d] Decode text");
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

        if !transform_ops.contains(&operation) {
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
        let key: u32 = key.trim().parse().expect("Invalid key.");
        
        match operation {
            'e' => println!("\nEncoded text: {}", encode(text.as_str(), key)),
            'd' => println!("\nDecoded text: {}", decode(text.as_str(), key)),
            'q' => break,
            _ => ()
        };
    }
}

fn main() {
    println!("--- Caesar Cipher ---\n");

    caesar_cipher();
}
