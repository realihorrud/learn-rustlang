fn main() {
    // integer types includes: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    // let mut age: u8 = 20;
    // println!("Age is: {age}");
    // age = age + 240; // attempt to add with overflow

    // float-point number includes: f32, f64
    let x = 2.0; // f64

    // numeric operations
    println!("{}", 10 / 3);
    println!("{}", 10.0 / 3.0);

    // bool type
    let is_learning_rust: bool = true;

    // char type
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Characters: {z}, {heart_eyed_cat}");

    // the tuple type
    let tup: (i32, f32, u32) =  (100, 3.14, 5000);
    let (x, y, z) = tup; // destructuring the tuple
    println!("Values are: {x} ,{y}, {z}");
    println!("First value: {}", tup.0);
    // The tuple without any values has a special name, _unit_

    // array and vector types
    let names = ["Alice", "Bob", "Eva"];
    let a = [1; 100]; // an array of 100 elements, all initialized to 1
    println!("First name is: {}", names[0]);
}
