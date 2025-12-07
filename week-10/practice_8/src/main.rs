// Declare a structure
struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

fn display(emp: Employee) {
    // Fetch values of specific structure fields using the . operator and print it
    println!("Name is :{} company is {} age is {}", emp.ceo, emp.company, emp.age);
}

fn main() {
    // Initialize a structure
    let emp1 = Employee {
        company: String::from("Microsoft Corporation"),
        ceo: String::from("Satya Nadella"),
        age: 56,
    };

    // Initialize another structure
    let emp2 = Employee {
        company: String::from("Google Inc."),
        ceo: String::from("Sundai Pichai"),
        age: 51,
    };

    // Pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}