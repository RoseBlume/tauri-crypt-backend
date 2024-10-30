use rand::seq::SliceRandom;
use std::collections::HashMap;

pub fn encrypt_mono(text: &str) -> HashMap<char, char> {
    let alphabet: Vec<char> = ('A'..='Z').collect();
    let mut shuffled_alphabet = alphabet.clone();
    let mut rng = rand::thread_rng();
    shuffled_alphabet.shuffle(&mut rng);

    let cipher_map: HashMap<char, char> = alphabet.iter().cloned().zip(shuffled_alphabet.iter().cloned()).collect();

    let input_upper = text.to_uppercase();
    let cipher_text: String = input_upper.chars().map(|c| {
        if c.is_alphabetic() {
            *cipher_map.get(&c).unwrap_or(&c)
        } else {
            c
        }
    }).collect();

    println!("Original Alphabet: {:?}", alphabet);
    println!("Cipher Alphabet: {:?}", shuffled_alphabet);
    println!("{}", input_upper);
    println!("{}", cipher_text);

    cipher_map
}
pub fn encrypt_vigenere(text: &str, key: &str) -> String {
    let alphabet: Vec<char> = ('A'..='Z').collect();
    let input_upper = text.to_uppercase();
    let key_upper = key.to_uppercase();
    let key_len = key_upper.len();

    let cipher_text: String = input_upper.chars().enumerate().map(|(i, c)| {
        if c.is_alphabetic() {
            let text_pos = alphabet.iter().position(|&x| x == c).unwrap();
            let key_pos = alphabet.iter().position(|&x| x == key_upper.chars().nth(i % key_len).unwrap()).unwrap();
            alphabet[(text_pos + key_pos) % 26]
        } else {
            c
        }
    }).collect();

    println!("Key: {}", key_upper);
    println!("Input Text: {}", input_upper);
    println!("Cipher Text: {}", cipher_text);

    cipher_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_and_print() {
        let text = "Hello, World! 123";
        let key = encrypt_mono(text);
        assert_eq!(key.len(), 26); // Ensure the key has 26 entries
    }
}