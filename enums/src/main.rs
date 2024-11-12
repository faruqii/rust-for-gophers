/*
Enums are a way to define a type by enumerating its possible values.
Each value can have data associated with it.
Enums are useful for defining types that can have a fixed set of values.
In rust, enums are defined using the enum keyword.
Rust enums can declare in various ways:
1. Simple Enum
2. Enum with Data
3. Enum with Methods
*/

// 1. Simple Enum
// Enums can be used to define a type that can have a fixed set of values.
enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

// 2. Enum with Data
// Enums can also have data associated with each variant.
enum Animal {
    Mammal { habitat: String },
    Bird { habitat: String },
    Reptile { habitat: String },
}

// 3. Enum with Methods
// Enums can have methods associated with them.
enum Vehicle {
    Car { brand: String, year: u32 },
    Truck { brand: String, capacity_tons: f64 },
    Motorcycle { brand: String, cc: u32 },
}

impl Vehicle {
    fn display_info(&self) {
        match self {
            Vehicle::Car { brand, year } => {
                println!("Car - Brand: {}, Year: {}", brand, year);
            }
            Vehicle::Truck {
                brand,
                capacity_tons,
            } => {
                println!("Truck - Brand: {}, Capacity: {} tons", brand, capacity_tons);
            }
            Vehicle::Motorcycle { brand, cc } => {
                println!("Motorcycle - Brand: {}, CC: {}", brand, cc);
            }
        }
    }
}

fn main() {
    let vehicle = VehicleType::Car;
    match vehicle {
        VehicleType::Car => println!("Vehicle Type: Car"),
        VehicleType::Truck => println!("Vehicle Type: Truck"),
        VehicleType::Motorcycle => println!("Vehicle Type: Motorcycle"),
    }

    let animal = Animal::Mammal {
        habitat: String::from("Land"),
    };
    match animal {
        Animal::Mammal { habitat } => println!("Mammal - Habitat: {}", habitat),
        Animal::Bird { habitat } => println!("Bird - Habitat: {}", habitat),
        Animal::Reptile { habitat } => println!("Reptile - Habitat: {}", habitat),
    }

    let car = Vehicle::Car {
        brand: String::from("Toyota"),
        year: 2019,
    };
    let truck = Vehicle::Truck {
        brand: String::from("Ford"),
        capacity_tons: 2.5,
    };
    let motorcycle = Vehicle::Motorcycle {
        brand: String::from("Honda"),
        cc: 250,
    };
    car.display_info();
    truck.display_info();
    motorcycle.display_info();
}
