fn main() {
    // let mut s = String::from("");
    // s.push_str("Hello, Rust!");
    // println!("{s}");
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}, world!"); // error: borrow of moved value: `s1`
    
    let mut hello1 = String::from("Hello!");
    hello1 = String::from("Привіт!");
    println!("{hello1}");

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}"); // error: borrow of moved value: `s`

    let s3 = String::from("hello, world!");
    let (s1, length) = calculate_length(s3);
    println!("The length of '{}' is {}.", s1, length);

    let x = 10;
    makes_copy(x);
    println!("{x}"); // This works because integers implement the Copy trait
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: u8) {
    println!("{some_integer}");
}
