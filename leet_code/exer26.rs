//2037. Minimum Number of Moves to Seat Everyone

pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();
    seats
        .iter()
        .zip(students.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn main() {
    println!(
        "{}",
        min_moves_to_seat([3, 1, 5].to_vec(), [2, 7, 4].to_vec())
    );
}
