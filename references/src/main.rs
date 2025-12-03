fn main() {
    let mut s1 = String::from("Hello, world!");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    change_given_string(&mut s1);

    println!("After change, the string has become: '{s1}'");
}

fn change_given_string(s: &mut String) {
    s.push_str(" - modified");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
