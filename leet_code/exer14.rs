//2114. Maximum Number of Words Found in Sentences

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let mut num_words: Vec<i32> = Vec::new();
    for word in sentences {
        num_words.push(word.matches(' ').count() as i32 + 1);
    }
    let ans = *num_words.iter().max().expect("Error");
    ans
}

fn main() {
    println!(
        "{}",
        most_words_found(
            [
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ]
            .to_vec()
        )
    );
}
