use std::{collections::HashMap, vec};

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // vectors
    let _a = [1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("This is the third item: {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Element not found"),
    }

    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Element not found"),
    }

    for i in &mut v {
        *i += 50
    }
    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(12.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer"),
    }
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer"),
    }

    // strings - stored as a collection of UTF-8 encoded bytes
    let _s1 = String::new();
    let s2 = "initial contents";
    let _s3 = s2.to_string();

    let mut s4 = String::from("initial contents");

    s4.push_str(" bar");
    s4.push('!');
    println!("{}", s4);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s4 = format!("{}{}", s1, s2);
    println!("{}", s4);

    // indexing into a string
    let hello: String = String::from("hello");
    // let c: char = hello[0]; // wrong because different characters take different amounts of memory - this would get the first byte!

    for b in "hellošđčćž".bytes() {
        println!("{}", b);
    }
    println!("");
    for c in "hellošđčćž".chars() {
        println!("{}", c);
    }
    println!("");
    for g in "hellošđčćž".graphemes(true) {
        println!("{}", g);
    }

    // hashmaps
    println!();
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 3);
    scores.insert(yellow, 7);

    let blue_score = scores.get("blue");
    println!("{}", blue_score.unwrap());

    let mut scores = HashMap::new();
    scores.insert("one", 1);
    scores.insert("one", 2);

    scores.entry("two").or_insert(15);
    scores.entry("two").or_insert(25);

    for (k, v) in scores {
        println!("{} {}", k, v);
    }

    let text = "hello world wonderful world";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0); // find existing value for this key, or insert 0 as a default => return a mutable reference to the value!
        *count += 1; // we can dereference this and modify it
    }
    println!("{:?}", words);
}
