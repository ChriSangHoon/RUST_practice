//771. Jewels and Stones

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jew: Vec<_> = jewels.chars().collect();
    let mut sum: i32 = 0;
    for x in jew {
        sum += stones.matches(x).count() as i32;
    }
    sum
}

fn main() {
    println!(
        "{}",
        num_jewels_in_stones("aA".to_string(), "aAAbbb".to_string())
    )
}
