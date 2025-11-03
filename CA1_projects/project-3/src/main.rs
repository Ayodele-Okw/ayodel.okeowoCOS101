//Rust program to manage simple purchase orders for a computer store 

use std::io;

fn main() {
    loop {
        println!("Code Item Price (₦)
1 Laptop 550,000
2 Monitor 120,000
3 Keyboard 15,000
4 Headset 25,000");

        

        let mut input1 = String::new();
        println!("Enter an item code");
        io::stdin().read_line(&mut input1).expect("invalid string");
        let code:i32 = input1.trim().parse().expect("invalid input");

        let mut input2 = String::new();
        println!("Enter quantity");
        io::stdin().read_line(&mut input2).expect("invalid string");
        let quantity:i32 = input2.trim().parse().expect("invalid input");

        let l:i32 = 550000;
        let m:i32 = 120000;
        let k:i32 = 15000;
        let h:i32 = 25000;

        let total = quantity * code;

        if code == 1  {
            let total = quantity * 550000;
        }else if code == 2  {
            let total = quantity * 120000;

        }else if code == 3  {
            let total = quantity * 15000;
        }else if code == 4  {
            let total = quantity * 25000;
        } if total > 500000 {
            let total = total - ((7/100) * total);

        }if code == '5' as i32  {
            break
        }

        println!(" Your total price is {}", total);
        println!("if you want to end the program type 5");




        



    }
}