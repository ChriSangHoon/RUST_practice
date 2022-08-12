//771. Jewels and Stones

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let sum: i32 = stones.chars().filter(|&x| jewels.contains(x)).count() as i32;
    sum
}

fn main() {
    println!(
        "{}",
        num_jewels_in_stones("aA".to_string(), "aAAbbb".to_string())
    )
}
