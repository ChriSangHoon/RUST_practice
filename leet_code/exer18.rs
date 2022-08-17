//1313. Decompress Run-Length Encoded List

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();

    for i in 0..nums.len() / 2 {
        let first = nums[2 * i];
        let second = nums[2 * i + 1];

        for i in 0..first {
            ans.push(second);
        }
    }
    ans
}

fn main() {
    println!("{:?}", decompress_rl_elist([1, 2, 3, 4].to_vec()))
}
