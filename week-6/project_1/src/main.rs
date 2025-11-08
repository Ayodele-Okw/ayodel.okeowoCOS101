use std::io;

fn main() {
    loop {
         let mut food_choice = String::new();
        let mut quantity = String::new();


        println!("


            MENU                           PRICE
            P = Poundo Yam/Edinkaiko Soup   -N3,200
            F = Fried Rice & Chicekn        -N3,000
            A = Amala & Ewedu Soup          -N2,500
            E = Eba & Egusi Soup            -N2,000
            W = White Rice & Stew           -N2,500
            Enter a letter to make your choice");
        io::stdin().read_line(&mut food_choice).expect("not a valid string");
        let food_choice = food_choice.trim().chars().next();

        println!("how much of this would you like? ( 5 portions at max)");
        io::stdin().read_line(&mut quantity).expect("not a valid string");
        let quantity:f32 = quantity.trim().parse().expect("invalid number");

        
        
        


        if food_choice == Some('P') {
            let price:f32= 3200.0;
            let total_charges = quantity * price;
            if quantity <= 5.0 {
                if total_charges > 10000.0 {
                    let new_charges:f32 = total_charges - ((5.0/100.0) * total_charges);
                    println!("since your order is greater than 10000 you get a discount of 5% your total charges now is: {}", new_charges);
                    break;
                }else if total_charges < 10000.0 {
                    println!("your total charges is = {}", total_charges);
                break;
                }
                
            }else if quantity > 5.0 {
                println!("You have exceeded the portions limit");
                break;
            }

        }else if food_choice == Some('F') {
            let price:f32= 3000.0;
            let total_charges = quantity * price;
            if quantity <= 5.0 {
                if total_charges > 10000.0 {
                    let new_charges:f32 = total_charges - ((5.0/100.0) * total_charges);
                    println!("since your order is greater than 10000 you get a discount of 5% your total charges now is: {}", new_charges);
                    break;
                }else if total_charges < 10000.0 {
                    println!("your total charges is = {}", total_charges);
                break;
                }
            }else if quantity > 5.0 {
                println!("You have exceeded the portions limit");
                break;
            }

        }else if food_choice == Some('A') {
            let price:f32= 2500.0;
            let total_charges = quantity * price;
            if quantity <= 5.0 {
                 if total_charges > 10000.0 {
                    let new_charges:f32 = total_charges - ((5.0/100.0) * total_charges);
                    println!("since your order is greater than 10000 you get a discount of 5% your total charges now is: {}", new_charges);
                    break;
                }else if total_charges < 10000.0 {
                    println!("your total charges is = {}", total_charges);
                break;
                }
            }else if quantity > 5.0 {
                println!("You have exceeded the portions limit");
                break;
            }
        }else if food_choice == Some('E') {
            let price:f32= 2000.0;
            let total_charges = quantity * price;
            if quantity <= 5.0 {
                 if total_charges > 10000.0 {
                    let new_charges:f32 = total_charges - ((5.0/100.0) * total_charges);
                    println!("since your order is greater than 10000 you get a discount of 5% your total charges now is: {}", new_charges);
                    break;
                }else if total_charges < 10000.0 {
                    println!("your total charges is = {}", total_charges);
                break;
                }
            }else if quantity > 5.0 {
                println!("You have exceeded the portions limit");
                break;
            }
        }else if food_choice == Some('W') {
            let price:f32= 2500.0;
            let total_charges = quantity * price;
            if quantity <= 5.0 {
                if total_charges > 10000.0 {
                    let new_charges:f32 = total_charges - ((5.0/100.0) * total_charges);
                    println!("since your order is greater than 10000 you get a discount of 5% your total charges now is: {}", new_charges);
                    break;
                }else if total_charges < 10000.0 {
                    println!("your total charges is = {}", total_charges);
                break;
                }
            }else if quantity > 5.0 {
                println!("You have exceeded the portions limit");
                break;
            }
        }
    }
}