use std::io;

fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("\nThe City vector has element {}", city.len());

    // Push new elements into
    let mut input1 = String::new();
    
    println!("How many Cities do you want to enter?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    
    let city_num_i32: i32 = input1.trim().parse().expect("Invalid input");
    let city_num = city_num_i32 as usize;

    for count in 0..city_num {
        println!("Enter City {}", count + 1);
        
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        
        // Use to_string() to convert the input into a String element
        let new_city = input2.trim().to_string(); 
        city.push(new_city);
    }
    
    println!("\nYour preferred cities are \n");
    
    // loop to iterate elements in vector
    let mut count = 1;
    for i in city {
        // iterating through i on the vector
        println!("{}: {}", count, i);
        count+=1;
    }
}