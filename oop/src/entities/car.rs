/*
Rust does not have a built-in support for object-oriented programming (OOP) like other languages such as C++ or Java. However, Rust provides a way to implement OOP concepts using structs and traits.
By default structs is private, so we need to use pub keyword to make it public.
Below is an example of a simple Car struct with an associated Engine struct.
*/

pub struct Car {
    brand: String,
    model: String,
    year: u32,
    color: String,
    price: f64,
    engine: Engine,
}

pub struct Engine {
    pub fuel_type: String,
    pub displacement: f64,
    pub power: f64,
}

// Implementation of the Car struct
impl Car {
    // Constructor method for the Car struct
    pub fn new(brand: &str, model: &str, year: u32, color: &str, price: f64, engine: Engine) -> Car {
        Car {
            brand: String::from(brand),
            model: String::from(model),
            year,
            color: String::from(color),
            price,
            engine,
        }
    }

    // Method to display the details of the car
    pub fn display(&self) {
        println!("Brand: {}", self.brand);
        println!("Model: {}", self.model);
        println!("Year: {}", self.year);
        println!("Color: {}", self.color);
        println!("Price: ${}", self.price);
        println!("Engine: {} {}L, {} hp", self.engine.fuel_type, self.engine.displacement, self.engine.power);
    }
}