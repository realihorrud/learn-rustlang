const FIVE_HOURS_IN_SECONDS: u32 = 60 * 60 * 5;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is: {x}");

    println!("Five hours in seconds is: {}", FIVE_HOURS_IN_SECONDS);
    
    // Shadowing

    let y = 1;

    let y = y + 5;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let name = "Alice";
    let name = name.len();
    println!("The length of the name is: {name}"); // works fine due to shadowing

    // let mut foo = "Hello, Rust!";
    // foo = foo.len(); // compile-time error
}
