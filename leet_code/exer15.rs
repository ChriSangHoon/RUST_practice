pub fn sort_sentence(s: String) -> String {
    let mut split_sentece: Vec<_> = s
        .split_whitespace()
        .map(|x| {
            (
                x[..x.len() - 1].to_string(),
                x[x.len() - 1..].parse::<usize>().expect("Error"),
            )
        })
        .collect();

    split_sentece.sort_by_key(|x| x.1);
    let ans: String = split_sentece
        .iter()
        .map(|x| x.0.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    ans
}

fn main() {
    println!("{}", sort_sentence("is2 sentence4 This1 a3".to_string()));
}
