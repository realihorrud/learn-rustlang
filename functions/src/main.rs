fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Add function: with x: 10_000 and y: 20_000 = {}", add(10_000, 20_000));

    another_function(50, '!');
}

fn another_function(x: u8, c: char) {
    println!("x: {x}, c: {c}");
}
