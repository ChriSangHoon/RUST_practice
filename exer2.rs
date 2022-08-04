// 1920. Build Array from Permutation

pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();

    for (i, _el) in nums.iter().enumerate() {
        let temp = nums[i];
        ans.push(nums[temp as usize]);
    }
    ans
}

fn main() {
    println!("{:?}", build_array([0, 2, 1, 5, 3, 4].to_vec()));
}
