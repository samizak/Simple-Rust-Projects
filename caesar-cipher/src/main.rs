fn main() {
    println!("Hello, world!");

    let string_to_encrypt = "hello world".to_uppercase();
    let string_vec: Vec<char> = string_to_encrypt.chars().collect();
    let shift: usize = 5;

    let possible_chars: Vec<char> = ('A'..='Z').into_iter().collect::<Vec<char>>();
    let mut encrypted: Vec<char> = Vec::new();

    // let x = string_vec.iter().map(|c| {
    //     if possible_chars.contains(&c) {
    //         let shifted_char = (&c as u8) + shift as u8;
    //         return shifted_char as char;
    //     } else {
    //         return &c as char;
    //     }
    // });

    for c in string_vec {
        if possible_chars.contains(&c) {
            let mut shifted_char = (c as u8) + shift as u8;
            if shifted_char > ('Z' as u8) {
                shifted_char -= ('A' as u8) + ('Z' as u8) - shifted_char;
            }
            encrypted.push(shifted_char as char);
        } else {
            encrypted.push(c as char);
        }
    }

    let mut decrypted: Vec<char> = Vec::new();
    for c in encrypted.clone() {
        if possible_chars.contains(&c) {
            let mut shifted_char = (c as u8) - shift as u8;
            if shifted_char < ('A' as u8) {
                shifted_char += ('Z' as u8) - ('A' as u8) - shifted_char;
            }
            decrypted.push(shifted_char as char);
        } else {
            decrypted.push(c as char);
        }
    }

    let test: String = encrypted.iter().collect();
    let test2: String = decrypted.iter().collect();

    println!("Before: {}", string_to_encrypt);
    println!("After: {}", test);

    println!("descrypted: {}", test2);
}
