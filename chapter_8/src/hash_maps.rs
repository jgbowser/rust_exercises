// 8.3 Storing Keys with Associated Values in Hash Maps

// The last of the common collections is the Hash Map. the type HashMap<K, V>
// stores a mapping of keys of type K to values of type V using a hashing function
// which determines how it places these keys and values into memory.
// This is equivalent to an Object in JavaScript

pub fn run() {
    // Creating a New Hash Map
    use std::collections::HashMap; // least used, so isn't included in the prelude

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Hash Map of team scores: {:?}", scores);

    // Hash Maps are like vectors in that they must contain homogenous
    // All keys must be of the same type, same with values.

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name);
    println!("the score for the {team_name} team is: {:?}", team_score);

    // we can iterate over Hash Maps as well in Rust, nice! It looks like:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // this will print the keys in arbitrary order since Hash Maps aren't ordered

    // Hash Maps and Ownership
    // Types that implement the Copy trait, like i32, are copied into the hash map.
    // For owned values, like Strings, they are moved and the hash map becomes the owner
    let field_name = String::from("favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map);
    // field_name and field_value are invalid at this point, trying to use them
    // will result in a compilation error at this point.
    // we can pass references in, though that comes with potential issues discussed in Ch 10

    // Updating a Hash Map: Overwrite, add if not present, or Update

    // Overwrite, inserting a new value into an existing string
    // up above the key "Blue" already holds the value 10, let's overwrite it
    scores.insert(String::from("Blue"), 42);
    println!("Originally blue team had 10 points, that got overwritten to {:?}", scores.get("Blue"));

    // Adding a key only if it isn't present already
    // special method for this called `entry`
    let existing = scores.entry(String::from("Yellow")).or_insert(33); // Yellow exists, so nothing happens
    println!("entry returns a mutable reference for entries that exist: {existing}");
    let not_existing = scores.entry(String::from("Green")).or_insert(60); // Green doesn't exist, insert it with value 60
    println!("when an entry doesn't exist, it returns: {not_existing}. The inserted value");
    println!("The scores hash map should now have blue, yellow and green: {:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut char_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = char_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("char map: {:?}", char_map);
}