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

// ownership
enum Light {
    Bright,
    Dull,
}

// a - 'dull' variable passed to 'display_light' is now owned by 'display_light' function, and called 'light'
// a - once function 'display_light' finishes, it deletes the 'light(dull)' variable
// fn display_light(light:Light) {

// b - adding & to Light, indicating that the parameter is only a refernce of 'dull', and the owner is still 'main' function
fn display_light(light: &Light) {

    // a - 'light' is used by the match function
    // b - a reference of 'dull' is used in theh match function
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }    
} // a - function ends, and 'light' variable is deleted, and cannot be called into the function 'display_light' again
// b - function ends, and 'dull' is returned to 'main' function, as 'display_light' is not the owner of 'dull'

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

    // ownership
    // Legend:
    // a : not utilizing borrow
    // b : utilizing borrow

    // dull variable is owned by main function, so it cannot be called more than once
    let dull = Light::Dull;
    // a - dull variable was passed to the 'display_light' function
    // display_light(dull);
    // a -calling 'display_light' and passing 'dull' variable would faiil, because 'dull' is deleted on first 'display_light' call
    // display_light(dull);

    // b - adding and & to 'dull' which anchors the dull variable to make function main as the owner
    // b - pass a reference of 'dull' to 'display_light' function
    display_light(&dull);
    display_light(&dull);

}
