//1365. How Many Numbers Are Smaller Than the Current Number

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = Vec::new();
    for x in nums.iter() {
        let mut ans: i32 = 0;
        for j in nums.iter() {
            if x > j {
                ans += 1;
            }
        }
        answer.push(ans)
    }
    answer
}

fn main() {
    println!(
        "{:?}",
        smaller_numbers_than_current([8, 1, 2, 2, 3].to_vec())
    );
}
