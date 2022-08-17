//2325. Decode the Message

use std::collections::HashMap;

pub fn decode_message(key: String, message: String) -> String {
    let mut alphabet = 'a'..='z';
    let mut key_map = HashMap::new();
    for c in key.chars() {
        if c != ' ' && !key_map.contains_key(&c) {
            key_map.insert(c, alphabet.next().expect("Error"));
        }
    }
    message
        .chars()
        .map(|x| key_map.get(&x).unwrap_or(&' '))
        .collect::<String>()
}
fn main() {
    println!(
        "{}",
        decode_message(
            "the quick brown fox jumps over the lazy dog".to_string(),
            "vkbs bs t suepuv".to_string()
        )
    )
}
