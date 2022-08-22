//2011. Final Value of Variable After Performing Operations

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations
        .iter()
        .map(|x| if x.contains("+") { 1 } else { -1 })
        .sum()
}

fn main() {
    println!(
        "{}",
        final_value_after_operations(
            ["--X".to_string(), "X++".to_string(), "X++".to_string()].to_vec()
        )
    )
}
