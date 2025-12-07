struct Employee {
    name: String,
    company: String,
    age: u32,
}

fn main() {
    // Instantiate the Employee struct
    let emp1 = Employee {
        company: String::from("Enrst & Young"),
        name: String::from("Ebiong Jessica"),
        age: 25,
    };

    // Print the struct's fields
    println!("Name = {} \n", emp1.name);
    println!("Company = {} \n", emp1.company);
    println!("Age = {} ", emp1.age);
}