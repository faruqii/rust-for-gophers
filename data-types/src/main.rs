fn main() {
    /*
    Data Types in Rust:
    - Rust is a statically typed language, meaning that all variables must have a type at compile time.
    - Types can be explicitly specified, or inferred by the compiler.
    */

    // Integer types: i32, i64, u32, u64, etc.
    let signed_integer: i32 = -10; // signed 32-bit integer
    let unsigned_integer: u64 = 42; // unsigned 64-bit integer
    println!("Signed Integer: {}, Unsigned Integer: {}", signed_integer, unsigned_integer);

    // Floating-point types: f32, f64 (default is f64)
    let float_num: f64 = 3.14; // 64-bit floating-point number
    println!("Floating-point Number: {}", float_num);

    // Boolean type: true or false
    let is_active: bool = true;
    println!("Boolean: {}", is_active);

    // Character type: single Unicode character, uses single quotes
    let letter: char = 'A';
    println!("Character: {}", letter);

    // String types: `&str` (string slice) and `String` (owned string)
    let greeting: &str = "Hello, Rust!"; // string slice, immutable and fixed-length
    let name: String = String::from("Gopher"); // owned string, mutable and growable
    println!("String Slice: {}, Owned String: {}", greeting, name);

    // Array type: fixed-length, elements must be the same type
    let numbers: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", numbers);

    // Tuple type: can hold multiple values of different types
    let person: (i32, f64, &str) = (25, 175.5, "Alice");
    println!("Tuple: {:?}", person);
}
