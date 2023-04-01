use std::collections::HashMap;

pub fn hash_map_ex() {
    println!("\n====== Hashmap example ======");

    println!("******Create map");
    map_create();

    println!("******Get value from map");
    map_get();

    println!("******Iterate map");
    iterate_map();

    println!("******Ownership in map");
    ownership_map();

    println!("******Update map");
    update_map();
}

fn map_create() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Map of scores {:?}", scores)
}

fn map_get() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score is {}", score);
}

fn iterate_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn ownership_map() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("Map data ownership {:?}", map);
}

fn update_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Original map: {:?}", scores);

    scores.insert(String::from("Blue"), 25);
    println!("Updated map: {:?}", scores);

    scores.entry(String::from("Black")).or_insert(40);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Updated map when key exists: {:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Update map based on old value {:?}", map);
}
