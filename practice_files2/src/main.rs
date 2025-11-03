// Rust program to generate multiplication time table for numbers

use std::io;

fn main() {

    let mut input1 = String::new();

    println!("enter a number");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let number:i32 = input1.trim().parse().expect("not a valid number");

    //loop

    for multiple in 1..13 {
        let calc = number * multiple;
        println!("{} x {} = {}", input1, multiple, calc);
    }
}


