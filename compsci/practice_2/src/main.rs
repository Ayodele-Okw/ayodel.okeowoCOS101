// Rust program to calculate the area or volume of different shapes, depending on the user's choice.

use std::io;


fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let number: f64 = input.trim().parse().expect("Please enter a valid number");
    number
}

  fn trapezium() {
    let height = read_number("enter a value for height");
    let base1 = read_number("enter a value for base1");
    let base2 = read_number("enter a value for base2");

    let trapez_form = (height / 2.00 ) * ( base1 + base2);
    println!("the area of this trapezium is {}", trapez_form);
}


fn rhombus() {
    let diagonal1 = read_number("Enter a value for diagonal1");
    let diagonal2 = read_number("Enter a value for diagonal2");

    let rhom_form = (diagonal1 * diagonal2) / 2.00;
    println!("the area of this Rhombus is {}", rhom_form);
}


fn parallelogram() {

    let base = read_number("Enter a value for base");
    let altitude = read_number("Enter a value for altitude");

    let paral_form = base * altitude;
    println!("the area of this parallelogram is {}", paral_form);

}

fn cube() {
    let length = read_number("enter a value for length");


    let cube_form = 6.0 * (length).powf(2.0);
    println!("the area of the cube is {}", cube_form);
}

fn cylinder() {
    let radius = read_number("enter a value for radius");
    let height2 = read_number("enter a value for height2");

    let pi = std::f64::consts::PI;
    let cylin_form = pi * radius.powf(2.0) * height2;
    println!("Volume of cylinder = {}", cylin_form);
}


fn main() {

       println!("choose a calculation:
                1.Area of a trapezium
                2.Area of a Rhombus
                3.Area of a Parallelogram
                4.Area of a Cube
                5.Volume of a Cylinder");


    let mut user_choice = String::from("");
io::stdin().read_line(&mut user_choice).expect("invalid input");
let user_choice = user_choice.trim().to_string();

match user_choice.as_str() {
    "trapezium" => trapezium(),
    "rhombus" => rhombus(),
    "parallelogram" => parallelogram(),
    "cube" => cube(),
    "cylinder" => cylinder(),
    "1" => trapezium(),
    "2" => rhombus(),
    "3" => parallelogram(),
    "4" => cube(),
    "5" => cylinder(),
    _=> println!("invalid choice!"),
}


}


 







