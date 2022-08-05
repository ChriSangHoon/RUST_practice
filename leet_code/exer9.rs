//1281. Subtract the Product and Sum of Digits of an Integer

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let str: String = n.to_string();
    let mut vec: Vec<char> = Vec::new();
    for c in str.chars() {
        vec.push(c);
    }

    let mut mul: i32 = 1;
    let mut sum: i32 = 0;
    for i in 0..vec.len() {
        mul *= vec[i].to_digit(10).expect("Error") as i32;
        sum += vec[i].to_digit(10).expect("Error") as i32;
    }
    mul - sum
}

fn main() {
    println!("{}", subtract_product_and_sum(4421));
}
