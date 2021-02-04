use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

fn main() {
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();

        map.insert(field_name, field_value);

        // field_name; // bang, map owned
    }
    {
        let scores = gimme_hash_map();

        let blue_score = scores.get("Blue");

        println!("Blue score: {}", blue_score.unwrap());
    }
    {
        for (key, value) in &gimme_hash_map() {
            println!("{}: {}", key, value);
        }
    }
    {
        println!(
            "{}",
            match gimme_hash_map().entry(String::from("Blue")) {
                Occupied(_) => "occupied one ",
                Vacant(_) => "vacant one",
            }
        );
    }
}

fn gimme_hash_map() -> HashMap<std::string::String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);

    scores
}
