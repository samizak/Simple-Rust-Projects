pub fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' } as i32;
                let offset = (c as i32 - base + shift).rem_euclid(26);
                (base + offset) as u8 as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let text = "Hello, World!";
    let shift = 3;

    let encrypted = caesar_cipher(text, shift);
    let decrypted = caesar_cipher(&encrypted, -shift);

    println!("Original:  {}", text);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
