mod entities;

fn main() {
    // Create a new Car instance
    let car = entities::Car::new(
        "Toyota",
        "Corolla",
        2019,
        "Red",
        15000.0,
        entities::Engine {
            fuel_type: String::from("Gasoline"),
            displacement: 1.8,
            power: 140.0,
        },
    );

    // call the display method on the car instance
    car.display();
}
