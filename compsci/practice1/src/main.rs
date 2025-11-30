use std::io;

fn main() {

    println!(" 

            MENU                        PRICE
        
        P = poundo Yam/Edinkaiko Soup  -N3200
        F = Fried Rice & Chicken       -N3000
        A = Amala & Ewedu Soup         -N2500
        E = Eba & Egusi Soup           -N2000
        W = White RIce & Stew          -N2500");


    let mut user_choice = String::new();


    println!("make your choice, which one would you like to have?");
    io::stdin().read_line(&mut user_choice).expect("invalid input");
    let user_choice = user_choice.trim().to_uppercase();
    let user_choice_char = user_choice.chars().next().expect("invalid input");

    let price = match user_choice_char {
        'P' => 3200,
        'F' => 3000,
        'A' => 2500,
        'E' => 2000,
        'W' => 2500,
        _=> {
            println!("invalid food type!");
            return;
        }
    };

    println!("Enter quantity: ");
    let mut user_quantity = String::new();
    io::stdin().read_line(&mut user_quantity).expect("invalid input");
    let user_quantity:i32 = user_quantity.trim().parse().expect("invalid input");

    let total = price * user_quantity;

    if total > 10000 {
        let discounted_price:f32 = total as f32 - (0.05 * total as f32);
        println!("you get a discount since your order is greater than 10,000. your price is {}",discounted_price);
    }else if total <= 10000 {
        println!(" your total price is {}",total);
    }

    




   
}
