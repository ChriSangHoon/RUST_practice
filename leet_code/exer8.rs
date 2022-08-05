// 1431. Kids With the Greatest Number of Candies

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut ans: Vec<bool> = Vec::new();
    for i in 0..candies.len() {
        let mut temp_candies = candies.clone();
        temp_candies[i] += extra_candies;
        if temp_candies[i] == *temp_candies.iter().max().expect("Error") {
            ans.push(true);
        } else {
            ans.push(false);
        }
        // let mut max_candy = temp_candies.iter().max().expect("Error");
        // match temp_candies[i] {
        //     max_candy => ans.push(true),
        //     _ => ans.push(false),
        // }
    }
    ans
}

fn main() {
    println!("{:?}", kids_with_candies([2, 3, 5, 1, 3].to_vec(), 3));
}
