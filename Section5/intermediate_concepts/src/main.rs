enum Menu {
    Burger,
    Fries,
    Drink,
}

enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {

    // expression
    let my_num = 3;
    
    let is_lt_5 = if my_num < 5 { 
        true
    } else {
        false
    };

    let is_lt_5 = my_num < 5;

    println!("{:?}", is_lt_5);

    let message = match my_num {
        1 => "hello",
        _ => "goodbye",
    };

    println!("{:?}", message);


    // nested expressions
    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";

    let order_placed = match item {
        Menu::Drink => {
            if drink_type == "water" {
                println!("{:?}", drink_type);
                true
            } else {
                false
            }
        }
        _ => true,
    };

    println!("{:?}", order_placed);

    // demo: exxpressions

    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can access {:?}", can_access_file);

}