// Rust program to determine the annual incentive of 
// employees

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    

    println!("Enter age: ",);
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("not a valid number");

    println!(" Are you experienced yes or no: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience = input2.trim();

    if experience == "yes" {
        if age >= 40{
            println!("Your incentive is 1560000");
                }else if age >= 30 {
        println!("your incentive is 1480000 ");
    }else if age >= 28 {
        println!("your incentive is 1400000");
    }else if age < 28 {
        println!("your incentive is 1300000");
    
    }
    }else if experience == "no" {
         println!("your incentive is 100000");
        }

   


    


}