//1720. Decode XORed Array

pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    ans.push(first);
    for i in 0..encoded.len() {
        ans.push(encoded[i] ^ ans[i]);
    }
    ans
}

fn main() {
    println!("{:?}", decode([6, 2, 7, 3].to_vec(), 4))
}
