fn main() {
    let age = 20;

    if age >= 18 {
        println!("You're allowed to use this program.");
    } else {
        println!("Access denied. You must be at least 18 years old.");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // ternary-like expression using if
    let condition = true;
    let result = if condition { "Condition is true" } else { "Condition is false" };
    println!("{}", result);

    // loop {
    //     println!("I learn Rust!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter * 4;
        }
    };
    println!("The result is {}", result);

    let mut number = 10;
    while number != 100 {
        number += 10;
    }
    println!("Number is {number}");

    let a = [1, 2, 3, 4, 5];
    for i in a {
        println!("the value is: {i}");
    }

    for number in (1..4).rev() {
        if number != 1 {
            print!("{number}...");
        } else {
            print!("{number}");
        }
    }
    println!("");
}
