fn main() {
    /*
    Operators in Rust:
    - Rust supports basic arithmetic, comparison, logical, bitwise, and assignment operators.
    */

    // Arithmetic Operators
    let a = 10;
    let b = 3;
    println!("Addition: {} + {} = {}", a, b, a + b);           // Addition
    println!("Subtraction: {} - {} = {}", a, b, a - b);        // Subtraction
    println!("Multiplication: {} * {} = {}", a, b, a * b);     // Multiplication
    println!("Division: {} / {} = {}", a, b, a / b);           // Division (integer division)
    println!("Remainder: {} % {} = {}", a, b, a % b);          // Modulus
    println!();
    // Comparison Operators (Result is a boolean)
    println!("Equal: {} == {} is {}", a, b, a == b);           // Equal to
    println!("Not Equal: {} != {} is {}", a, b, a != b);       // Not equal to
    println!("Greater Than: {} > {} is {}", a, b, a > b);      // Greater than
    println!("Less Than: {} < {} is {}", a, b, a < b);         // Less than
    println!("Greater or Equal: {} >= {} is {}", a, b, a >= b); // Greater than or equal to
    println!("Less or Equal: {} <= {} is {}", a, b, a <= b);    // Less than or equal to
    println!();
    // Logical Operators
    let is_rust_fun = true;
    let is_go_fun = true;
    println!("AND: {} && {} = {}", is_rust_fun, is_go_fun, is_rust_fun && is_go_fun); // AND
    println!("OR: {} || {} = {}", is_rust_fun, is_go_fun, is_rust_fun || is_go_fun);  // OR
    println!("NOT: !{} = {}", is_rust_fun, !is_rust_fun);                             // NOT
    println!();
    // Bitwise Operators
    let x: u8 = 0b1100; // 12 in binary
    let y: u8 = 0b1010; // 10 in binary
    println!("Bitwise AND: {:08b} & {:08b} = {:08b}", x, y, x & y); // Bitwise AND
    println!("Bitwise OR: {:08b} | {:08b} = {:08b}", x, y, x | y);  // Bitwise OR
    println!("Bitwise XOR: {:08b} ^ {:08b} = {:08b}", x, y, x ^ y); // Bitwise XOR
    println!("Bitwise NOT: !{:08b} = {:08b}", x, !x);               // Bitwise NOT
    println!("Left Shift: {:08b} << 1 = {:08b}", x, x << 1);        // Left Shift
    println!("Right Shift: {:08b} >> 1 = {:08b}", x, x >> 1);       // Right Shift
    println!();
    // Assignment Operators
    let mut z = 5;
    println!("Initial value of z: {}", z);
    z += 2; // Equivalent to z = z + 2
    println!("After += 2: {}", z);
    z -= 1; // Equivalent to z = z - 1
    println!("After -= 1: {}", z);
    z *= 3; // Equivalent to z = z * 3
    println!("After *= 3: {}", z);
    z /= 2; // Equivalent to z = z / 2
    println!("After /= 2: {}", z);
    z %= 2; // Equivalent to z = z % 2
    println!("After %= 2: {}", z);
    println!();
    // Compound Assignment with Bitwise Operators
    let mut w = 0b1010; // 10 in binary
    println!("Initial value of w: {:08b}", w);
    w &= 0b1100; // Equivalent to w = w & 0b1100
    println!("After &= 0b1100: {:08b}", w);
    w |= 0b0011; // Equivalent to w = w | 0b0011
    println!("After |= 0b0011: {:08b}", w);
    w ^= 0b1111; // Equivalent to w = w ^ 0b1111
    println!("After ^= 0b1111: {:08b}", w);
    w <<= 1;     // Equivalent to w = w << 1
    println!("After <<= 1: {:08b}", w);
    w >>= 2;     // Equivalent to w = w >> 2
    println!("After >>= 2: {:08b}", w);
}
