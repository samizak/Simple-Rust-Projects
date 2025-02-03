fn main() {
    let shift: usize = 5;
    let string_to_encrypt = "hello world!";
    let string_vec: Vec<char> = string_to_encrypt.to_uppercase().chars().collect();

    let possible_chars: Vec<char> = ('A'..='Z').into_iter().collect::<Vec<char>>();

    let encrypted: Vec<char> = string_vec
        .iter()
        .map(|c| {
            if possible_chars.contains(c) {
                let index = (*c as u8 - b'A') as usize;
                let shifted_index = (index + shift) % 26;
                return (shifted_index as u8 + b'A') as char;
            } else {
                return *c;
            }
        })
        .collect();

    let decrypted: Vec<char> = encrypted
        .iter()
        .map(|c| {
            if possible_chars.contains(c) {
                let index = (*c as u8 - b'A') as usize;
                let shifted_index = (index + 26 - shift) % 26;
                return (shifted_index as u8 + b'A') as char;
            } else {
                return *c;
            }
        })
        .collect();

    let encrypted_str: String = encrypted.iter().collect();
    let decrypted_str: String = decrypted.iter().collect();

    println!("Before: {}", string_to_encrypt);
    println!("Encrypted: {}", encrypted_str);
    println!("Decrypted: {}", decrypted_str);
}
