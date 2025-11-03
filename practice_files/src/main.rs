//Rust program to determine body mass index

use std::io;

fn main() {

     let mut input1 = String::new();
     let mut input2 = String::new();


     println!("Enter weight");
     io::stdin().read_line(&mut input1).expect("not a valid string");
     let weight:f32 = input1.trim().parse().expect("not a valid number");

     println!("Enter height");
     io::stdin().read_line(&mut input2).expect("not a valid string");
     let height:f32 = input2.trim().parse().expect("not a valid number");


     // formula for BMI
     let bmi = weight / height.powf(2.0);

     // Weight classes

     if bmi < 18.5 {
        println!("You are underweight");

     }else if bmi < 24.9 {
        println!("You are normal weight");
     }else if bmi < 29.9 {
        println!("You are overweight");

     }else if bmi >= 30.0 {
        println!("you are obese");
     }

     

}