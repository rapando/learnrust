/*
1. Vector allows to store a variable number of values next to each other
2. String is a collection of characters
3. hash map stores values in key:value format
*/
#![allow(dead_code, unused_variables)]

use std::{fmt::format, hash::Hash};


fn main() {
    // 1. vectors
    let mut v: Vec<i32> = Vec::new();
    // we can also define a vector with values already
    // let v = vec![1,2,3];
    v.push(5);
    v.push(3);
    v.push(7);

    let third_value: &i32 = &v[2];
    println!("The third element in the vector is {third_value}");

    let first = &v[0];
    // v.push(6); // this will fail because, we're doing a mutable borrow here while in the previous line there is an immutable borrow
    println!("the first element is {first}");

    // using get is safer in case the index does not exist
    let third_value: Option<&i32> = v.get(2);
    match  third_value {
        Some(third_value) => println!("The third element is {third_value}"),
        None => println!("There is no third element"),
    }

    // iterating over a vector (immutable for just reading)
    println!("immutable iteration (read only)");
    for i in &v {
        println!("{i}");
    }

    println!("mutable iteration (for updates)");
    for i in &mut v { // mutable borrow
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    // if we want to store different types in a vector, we would do with an enum
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let  row :Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    // loop through the row details
    for i in &row {
        println!("{:?}", i);
    }

    // ------- STRINGS
    let mut s = String::new(); // creates a new empty String
    let data = "initial contents"; // this is a string slice
    s = data.to_string();
    println!("The string is : {s}");

    let s = String::from("hello world");
    println!("The new string is {s}");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("String after pushing: {s}");

    // we can also concatenate with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;  // s1's ownership has been moved here so it 
                                // can't be used anymore
    println!("The new string: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("The game is {s}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("slice: {s}");

    // indexing strings in rust is not possible, unless you loop through it as chars or bytes
    for c in s.chars() {
        println!("{c}");
    }

    // HashMaps
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Red"), 100);

    // lets say we want to get the value of the Blue team
    let team_name = String::from("Blue");
    let blue_score = scores.get(&team_name).copied().unwrap_or(0);
    // the get method returns an Option<&V> therefore, if there is no value it
    // will return None

    // iterate over each key:value pair
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("{scores:?}");
    scores.insert(String::from("Blue"), 0); // this overwrites the value of Blue
    println!("{scores:?}");

    // if we want to only insert if a value does not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // sometimes we want to update the values based on some parameter
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // here when we increment count, it increases its pointer 
        // (concept from other languages) therefore incrementing the value
    }
    println!("word count: {map:?}");



}
