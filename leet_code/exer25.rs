//1688. Count of Matches in Tournament

pub fn number_of_matches(mut n: i32) -> i32 {
    let mut matches = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
            matches += n;
        } else {
            n = n / 2 + 1;
            matches += n - 1;
        }
    }
    matches
}

fn main() {
    println!("{}", number_of_matches(7));
}
