// 1281. Subtract the Product and Sum of Digits of an Integer

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let string = n.to_string();
    let vec = string
        .chars()
        .map(|x| x.to_digit(10).expect("Error") as i32);
    let mul: i32 = vec.clone().product();
    let sum: i32 = vec.sum();
    mul - sum
}

fn main() {
    println!("{}", subtract_product_and_sum(4421));
}
