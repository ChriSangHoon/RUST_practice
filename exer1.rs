pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    nums.repeat(2)
}

fn main(){
    println!("{:?}", get_concatenation([1,2,3].to_vec()))
}
