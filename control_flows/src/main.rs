fn main() {
    /*
    Control Flows in Rust:
    - Rust provides several control flow structures: `if`, `else`, `loop`, `while`, and `for`.
    - Rust also includes pattern matching with `match` and `if let`.
    */

    // `if` and `else`
    let age = 18;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    // `if` with expressions (Rust's `if` can be an expression that returns a value)
    let number = if age >= 18 { 10 } else { 5 };
    println!("Number based on age: {}", number);

    // `loop` - Infinite loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            println!("Loop reached 3, breaking out!");
            break; // Breaks out of the infinite loop
        }
    }

    // `while` loop - Runs as long as a condition is true
    let mut countdown = 5;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // `for` loop - Iterating over a range
    for i in 1..4 { // `1..4` is a range from 1 to 3 (end is exclusive)
        println!("For loop iteration: {}", i);
    }

    // `for` loop with inclusive range
    for i in 1..=3 { // `1..=3` includes the end value (1 to 3)
        println!("Inclusive range iteration: {}", i);
    }

    // Iterating over a collection
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits.iter() {
        println!("Fruit: {}", fruit);
    }

    // `match` - Pattern matching (similar to a switch statement but more powerful)
    let day = "Tuesday";
    match day {
        "Monday" => println!("Start of the week!"),
        "Friday" => println!("Almost the weekend!"),
        "Saturday" | "Sunday" => println!("It's the weekend!"),
        _ => println!("Just another day."), // `_` is a catch-all pattern
    }

    // `if let` - A shorter way to handle `match` for a single pattern
    let some_value = Some(10);
    if let Some(val) = some_value {
        println!("The value is {}", val);
    } else {
        println!("No value found.");
    }

    // `while let` - Loops as long as a pattern continues to match
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 3 {
            println!("Greater than 3, stopping.");
            optional = None;
        } else {
            println!("i is {}", i);
            optional = Some(i + 1);
        }
    }
}
