use std::collections::HashMap;

// -------------------HASHMAPS-------------------
fn hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    for (key, value) in &scores {
        println!("{key} {value}")
    }
}
