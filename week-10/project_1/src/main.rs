// Use the standard library's PI constant for calculation
use std::f64::consts::PI;

// Define a struct for a Circle
struct Circle {
    radius: f64,
}

// Implement methods for the Circle struct
impl Circle {
    // A method to calculate the circumference of the circle: C = 2 * PI * r
    // Takes an immutable reference (&self) and returns an f64.
    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    // A method to calculate the area of the circle: A = PI * r^2
    // Takes an immutable reference (&self) and returns an f64.
    fn area(&self) -> f64 {
        PI * self.radius.powi(2) // .powi(2) is used for squaring
    }

    // A function (not a method, as it doesn't take &self) to create a new Circle
    // This is often called a constructor or an associated function.
    fn new(r: f64) -> Circle {
        Circle { radius: r }
    }
}

fn main() {
    // Instantiate a Circle using the 'new' associated function
    let circle_one = Circle::new(5.0);
    
    // Instantiate a Circle directly
    let circle_two = Circle { radius: 10.5 };

    // Calculate and print the results for the first circle
    println!("--- Circle One (Radius: {}) ---", circle_one.radius);
    println!("Circumference: {:.2}", circle_one.circumference()); // {:.2} for 2 decimal places
    println!("Area: {:.2}", circle_one.area());

    println!("\n--- Circle Two (Radius: {}) ---", circle_two.radius);
    println!("Circumference: {:.2}", circle_two.circumference());
    println!("Area: {:.2}", circle_two.area());
}