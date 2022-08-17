//1528. Shuffle String

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut temp: Vec<_> = indices.iter().zip(s.chars()).collect();
    temp.sort_by_key(|x| x.0);
    let ans: String = temp.iter().map(|x| x.1).collect();
    ans
}

fn main() {
    println!(
        "{}",
        restore_string("codeleet".to_string(), [4, 5, 6, 7, 0, 2, 1, 3].to_vec())
    );
}
