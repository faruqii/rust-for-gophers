fn main() {
    /*
    Functions in Rust:
    - Functions are defined using the `fn` keyword.
    - Parameters must have types specified, and return types are optional.
    - Functions can return values using the `->` syntax, but must omit the semicolon on the last line to return.
    */

    greet("Alice");

    let sum = add(5, 10);
    println!("Sum: {}", sum);

    let result = square(4);
    println!("Square: {}", result);

    // Using a function with no parameters
    print_hello();

    // Using closures (anonymous functions)
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("Multiplication using closure: {}", multiply(3, 4));

    // Passing a function as a parameter (higher-order function)
    let result = operate(3, 4, add);
    println!("Result of operate with add function: {}", result);

    // Using closures in higher-order functions
    let result = operate(3, 4, |a, b| a - b);
    println!("Result of operate with closure (subtraction): {}", result);
}

// 1. Simple Function with Parameters
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 2. Function with Parameters and Return Type
fn add(x: i32, y: i32) -> i32 {
    x + y // Last expression without semicolon is the return value
}

// 3. Function with Explicit Return
fn square(number: i32) -> i32 {
    return number * number; // Using `return` keyword explicitly
}

// 4. Function with No Parameters and No Return
fn print_hello() {
    println!("Hello from Rust!");
}

// 5. Higher-order Function (accepts a function as a parameter)
fn operate(x: i32, y: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(x, y)
}
