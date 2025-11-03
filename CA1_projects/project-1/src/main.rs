//Rust program to calculate the average score and grade of students

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();


    println!("What is your name?");
    io::stdin().read_line(&mut input1).expect("invalid string");
    let name = input1.trim();


    println!("Enter your test 1 score");
    io::stdin().read_line(&mut input2).expect("invalid string");
    let test1:f32 = input2.trim().parse().expect("invalid input");


    println!("Enter your test 2 score");
    io::stdin().read_line(&mut input3).expect("invalid string");
    let test2:f32 = input3.trim().parse().expect("invalid input");


    println!("Enter your test 3 score");
    io::stdin().read_line(&mut input4).expect("invalid string");
    let test3:f32 = input4.trim().parse().expect("invalid input");


    //average

    let average:f32 = (test1 + test2 + test3 ) / 3.0;

    //Grade score
    if average > 100.00{
       println!(" invalid score and invalid grade"); 
    }else if average == 100.00 {
        println!("{} got an A and your average score is {}", name, average);
    }else if average >= 70.00 {
        println!("{} you got an A and your average score is {}", name, average);

    }else if average == 69.00 {
        println!("{} you got a B and your average score is {}", name, average );
    }else if average >= 60.00 {
        println!("{} you got a b and your average score is {}", name, average);
    }else if average == 59.00 {
        println!("{} you got a C and you average score is {} ", name, average);
    }else if average >= 50.00 {
        println!("{} you got a C and your average score is {}", name, average);
    }else if average == 49.00 {
        println!("{} you got a D and your average score is {}", name, average);
    }else if average >= 45.00 {
        println!("{} you got a D and your average score is {}",name, average);
    }else if average == 44.00 {
        println!("{} you got an F and your average score is {}",name, average);
    }else if average >= 0.00{
        println!("{} you got an F and your average score is {}",name, average);
    }else if average < 0.00 {
        println!(" invalid grade and invalid score");
    } 

}