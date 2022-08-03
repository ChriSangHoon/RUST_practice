// 1480. Running Sum of 1d Array

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut temp = 0;
    let mut ans: Vec<i32> = Vec::new();
        
    for x in nums.iter(){
        temp += x;
        ans.push(temp);
    }
    ans
}

fn main() {
    println!("{:?}", running_sum([1,2,3,4,].to_vec()));
}