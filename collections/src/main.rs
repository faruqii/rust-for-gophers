fn main() {
    /*
    Data Structures in Rust:
    - Rust provides several ways to organize data, including arrays, tuples, vectors, and structs.
    */

    // 1. Arrays
    // Arrays have a fixed size and store elements of the same type.
    let numbers: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", numbers);
    println!("First element: {}", numbers[0]);

    // Array with the same repeated value
    let repeated_values = [0; 5]; // Array with 5 elements, all set to 0
    println!("Repeated values array: {:?}", repeated_values);

    // 2. Tuples
    // Tuples can store values of different types and have a fixed size.
    let person: (&str, i32, f64) = ("Alice", 30, 1.65);
    println!("Tuple: {:?}", person);
    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);

    // 3. Vectors
    // Vectors are similar to arrays but are dynamically sized (can grow or shrink).
    let mut fruits = vec!["apple", "banana"];
    fruits.push("cherry"); // Adding an element to the vector
    println!("Vector: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);

    // Removing an element
    fruits.pop();
    println!("After removing last element: {:?}", fruits);

    // 4. Structs (similar to objects in other languages)
    // Structs allow you to define custom data types with named fields.
    struct Person {
        name: String,
        age: u32,
        height: f64,
    }

    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        height: 1.65,
    };
    println!("Struct: {} is {} years old and {} meters tall.", alice.name, alice.age, alice.height);

    // 5. Option type (for handling optional values)
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;
    println!("Some value: {:?}", some_value);
    println!("No value: {:?}", no_value);

    if let Some(val) = some_value {
        println!("Value inside Option: {}", val);
    } else {
        println!("No value found");
    }
}
