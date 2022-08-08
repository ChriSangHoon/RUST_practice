//1365. How Many Numbers Are Smaller Than the Current Number

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .map(|i| nums.iter().filter(|j| j.lt(&i)).count() as i32)
        .collect()
}

fn main() {
    println!(
        "{:?}",
        smaller_numbers_than_current([8, 1, 2, 2, 3].to_vec())
    );
}
