// 2160. Minimum Sum of Four Digit Number After Splitting Digits

pub fn minimum_sum(num: i32) -> i32 {
    let mut temp_num: i32 = num.clone();
    let mut vec: Vec<i32> = Vec::new();
    while temp_num > 9 {
        vec.push(temp_num % 10);
        temp_num = temp_num / 10;
    }
    vec.push(temp_num);
    vec.sort();
    let ans: i32 = vec[0] * 10 + vec[1] * 10 + vec[2] + vec[3];
    ans
}

fn main() {
    println!("{}", minimum_sum(4009));
}
