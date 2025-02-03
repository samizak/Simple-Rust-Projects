fn main() {
    println!("Hello, world!");

    let string_to_encrypt = "hello world!".to_uppercase();
    let string_vec: Vec<char> = string_to_encrypt.chars().collect();
    let shift: usize = 5;

    let possible_chars: Vec<char> = ('A'..='Z').into_iter().collect::<Vec<char>>();
    let mut encrypted: Vec<char> = Vec::new();

    for c in &string_vec {
        if possible_chars.contains(&c) {
            let index = (*c as u8 - b'A') as usize;
            let shifted_index = (index + shift) % 26;
            encrypted.push((shifted_index as u8 + b'A') as char);
        } else {
            encrypted.push(*c);
        }
    }

    let mut decrypted: Vec<char> = Vec::new();
    for c in &encrypted.clone() {
        if possible_chars.contains(&c) {
            let index = (*c as u8 - b'A') as usize;
            let shifted_index = (index + 26 - shift) % 26;
            decrypted.push((shifted_index as u8 + b'A') as char);
        } else {
            decrypted.push(*c);
        }
    }

    let test: String = encrypted.iter().collect();
    let test2: String = decrypted.iter().collect();

    println!("Before: {}", string_to_encrypt);
    println!("After: {}", test);

    println!("descrypted: {}", test2);
}
