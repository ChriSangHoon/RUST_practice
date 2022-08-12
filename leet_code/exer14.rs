//2114. Maximum Number of Words Found in Sentences

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let ans = sentences
        .iter()
        .map(|x| x.split_whitespace().count())
        .max()
        .expect("Error") as i32;
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
