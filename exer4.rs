// 1672. Richest Customer Wealth

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut ans: Vec<i32> = vec![1,2,3];
    for x in accounts.iter() {
        ans.push(x.iter().sum())
    }
    let answer: i32 = *ans.iter().max().unwrap();
    answer
}

fn main() {
    println!("{}", maximum_wealth([[1,2,3].to_vec(),[3,2,1].to_vec()].to_vec()))
}