// 2160. Minimum Sum of Four Digit Number After Splitting Digits

pub fn minimum_sum(num: i32) -> i32 {
    let str: String = num.to_string();
    let mut vec: Vec<char> = Vec::new();
    for i in 0..str.len() {
        vec.push(str.chars().nth(i).expect("Error"));
    }
    vec.sort();
    let ans: i32 = vec[0].to_digit(10).expect("Error") as i32 * 10 as i32
        + vec[1].to_digit(10).expect("Error") as i32 * 10 as i32
        + vec[2].to_digit(10).expect("Error") as i32
        + vec[3].to_digit(10).expect("Error") as i32;
    ans
}

fn main() {
    println!("{}", minimum_sum(4009));
}
