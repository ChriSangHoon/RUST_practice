// 1431. Kids With the Greatest Number of Candies

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    candies
        .iter()
        .map(|i| i + extra_candies >= *candies.iter().max().expect("Error"))
        .collect()
}

fn main() {
    println!("{:?}", kids_with_candies([2, 3, 5, 1, 3].to_vec(), 3));
}
