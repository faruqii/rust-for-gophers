fn main() {
    /* 
    Variables in Rust:
    - Variables are **immutable** by default.
    - Use the `mut` keyword to make a variable mutable.
    - Use `const` to define constants, which are always immutable and require an explicit type.
    */

    // Immutable variable: cannot be changed once assigned
    let age = 20;
    println!("Age (immutable): {}", age);
    // age = 21; // <-- Uncommenting this line will cause an error, as `age` is immutable

    // Mutable variable: use `mut` keyword to allow changes
    let mut age2 = 20;
    println!("Initial Age2 (mutable): {}", age2);
    age2 = 21; // This is allowed since age2 is mutable
    println!("Updated Age2: {}", age2);

    // Constant variable: use `const` keyword for values that won't change
    // - Constants must have explicit types and are always immutable
    const MAX_AGE: u32 = 100;
    println!("Max Age (constant): {}", MAX_AGE);

    // Variables declared with `let` can be shadowed
    // Shadowing allows us to reuse a variable name for different types or values
    let age = "twenty"; // Shadowing `age` with a different type
    println!("Age (shadowed with a string): {}", age);
}
