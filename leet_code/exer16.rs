// 1773. Count Items Matching a Rule

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let idx: usize = match &rule_key[..] {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => panic!("invalid rule_key"),
    };
    items.iter().filter(|x| x[idx] == rule_value).count() as i32
}

fn main() {
    println!(
        "{}",
        count_matches(
            [
                ["phone".to_string(), "blue".to_string(), "pixel".to_string()].to_vec(),
                [
                    "computer".to_string(),
                    "silver".to_string(),
                    "lenovo".to_string()
                ]
                .to_vec(),
                [
                    "phone".to_string(),
                    "gold".to_string(),
                    "iphone".to_string()
                ]
                .to_vec()
            ]
            .to_vec(),
            "color".to_string(),
            "silver".to_string()
        )
    );
}
