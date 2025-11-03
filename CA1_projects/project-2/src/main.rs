// Rust program to calculate compound interest and total amount

use std::io;

fn main() {

    loop {
        let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();



    println!("What is your principal");
    io::stdin().read_line(&mut input1).expect("invalid string");
    let principal:f32 = input1.trim().parse().expect("invalid input");
    if principal == 0.001 {
        break }

    println!("What is your rate");
    io::stdin().read_line(&mut input2).expect("invalid string");
    let rate:f32 = input2.trim().parse().expect("invalid input");


    println!("how long is the time");
    io::stdin().read_line(&mut input3).expect("invalid string");
    let time  = input3.trim().parse().expect("invalid input");


    //compound interest

    let a = principal * (1.00 + ( rate / 100.00)).powf(time);
    let ci = a - principal;
    println!("your amount is {} and your compound interest is {} ", a, ci);

    println!("if you want to end the program type 0.001 if not continue your calculations");

    


    }
    
}