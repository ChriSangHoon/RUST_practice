// 1470. Shuffle the Array

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut array1 : Vec<i32> = Vec::new();
    for i in 0..n{
        array1.push(nums[i as usize]);
        array1.push(nums[i as usize + n as usize])
    }
    array1
}

fn main() {
    println!("{:?}", shuffle([2,5,1,3,4,7].to_vec(), 3))
}
