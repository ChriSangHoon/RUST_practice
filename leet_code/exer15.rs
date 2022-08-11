pub fn sort_sentence(s: String) -> String {
    let mut word_vec = vec![""; s.split(" ").count()];
    for word_index in s.split(" ") {
        let (w, idx) = word_index.split_at(word_index.len() - 1);
        word_vec[idx.parse::<usize>().expect("Parsing error") - 1] = w;
    }
    word_vec.join(" ")
}

fn main() {
    println!("{}", sort_sentence("is2 sentence4 This1 a3".to_string()));
}
