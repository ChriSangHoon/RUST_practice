// 1512. Number of Good Pairs

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    for (i, num) in nums.iter().enumerate() {
        for j in nums[i + 1..].iter() {
            if num == j {
                ans += 1
            }
        }
    }
    ans
}

fn main() {
    println!("{}", num_identical_pairs([1, 1, 1, 1].to_vec()));
}
