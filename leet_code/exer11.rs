pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let sum_vec: Vec<i32> = accounts.into_iter().map(|x| x.iter().sum()).collect();
    let ans: i32 = *sum_vec.iter().max().expect("Error");
    ans
}

fn main() {
    println!(
        "{}",
        maximum_wealth([[1, 2, 3].to_vec(), [3, 2, 1].to_vec()].to_vec())
    )
}
