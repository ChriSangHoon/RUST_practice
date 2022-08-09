// 1512. Number of Good Pairs
// Constraints : 1 <= nums.length <= 100, 1 <= nums[i] <= 100
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let ans = nums
        .iter()
        .fold([0i32; 101].to_vec(), |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        })
        .iter()
        .filter(|&x| *x > 1)
        .map(|x| x * (x - 1) / 2)
        .sum();
    ans
}

fn main() {
    println!("{}", num_identical_pairs([1, 2, 3, 1, 1, 3].to_vec()));
}
