use std::collections::HashMap;

fn main() {
    // Vector
    let mut v: Vec<i32> = Vec::new();
    // or infer
    let _v1  = vec![1, 2, 3];

    // Add values
    v.push(1);
    v.push(2);
    v.push(3);

    // Index error example
    let v2 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // Method one (throws error)
    let does_not_exist = v.get(100); // Method two (returns None handled with match)

    // Index error not handled
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Indexing error handled
    match v.get(2){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let mut v3 = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // Index 0 borrowed immutable
    // v3.push(6);
    // println!("The first element is: {}", first); // Wont work! &v[0] has been borrowed
    
    // Immutable loop
    let v4 = vec![10, 50, 100];
    for i in &v4 {
        println!("{}", i);
    }

    // Mutable loop
    let mut v5 = vec![1, 2, 3];
    for i in &mut v5 {
        *i += 50;
        println!("{}", i);
    }

    // Can create an enum to be held in a vector
    enum SpredsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpredsheetCell::Int(3),
        SpredsheetCell::Float(10.12),
        SpredsheetCell::Text(String::from("blue")),
    ];

    for val in &row {
        match val {
            SpredsheetCell::Int(i) => println!("{}", i),
            _ => (), // Do nothing with other values
        }
    }


    // Strings
    let mut _s = String::new();
    // or if we have data
    let mut _s1 = "Hello".to_string();
    // or
    let mut s2 = String::from("Hello");

    // Append to string
    s2.push_str(" World");
    s2.push('!'); // Single char

    println!("{}", s2);

    // Plus (+) operator
    let s3 = String::from("Hello, ");
    let s4 = String::from("World!");
    let s5 = s3 + &s4; // s3 has been moved here, can no longer be used

    println!("{}", s5);

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");

    let p = format!("{}-{}-{}", s6, s7, s8);
    println!("{}", p);


    // Rust strings do not support indexing as they are UTF-8
    // s1[0]; Will not work

    // You can use this
    for c in "Hello".chars(){
        println!("{}", c);
    }

    // Key value store (HashMap)
    // HashMap<K, V>
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    scores.insert(String::from("Yellow"), 30); // This replaces Yellows value
    scores.entry(String::from("Red")).or_insert(50); // Only inserts if Red has no value

    // Loop through HashMap
    for (key, value) in &scores {
        println!("Key: {}\tValue: {}", key, value);
    }

    // Returns an option as there may or not be a value
    println!("{:?}", scores.get(&String::from("Blue"))); 

    // To vectors to HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];

    // Infer types with HashMap<_, _>
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Updating value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){ // Break sentance into words
        let count = map.entry(word).or_insert(0); // Retruns a mutable value
        *count += 1; // Dereference count and add one
    }

    for (key, value) in &map{
        println!("Key: {}\tValue: {}", key, value);
    }

}

