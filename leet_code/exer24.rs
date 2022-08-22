//1389. Create Target Array in the Given Order

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut ans_vec: Vec<i32> = Vec::new();
    for (x, y) in nums.iter().zip(index.iter()) {
        ans_vec.insert(*y as usize, *x);
    }
    ans_vec
}

fn main() {
    println!(
        "{:?}",
        create_target_array([0, 1, 2, 3, 4].to_vec(), [0, 1, 2, 2, 1].to_vec())
    );
}
