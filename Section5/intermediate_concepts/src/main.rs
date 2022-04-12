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


struct Book {
    pages: i32,
    rating: i32,
}

// parameters with '&' only gets the reference of the value, and not the original value
fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}


// implementing functions
struct Temperature {
    degrees_f: f64,
}

/* Original function, before changing to impl
fn show_temp(temp::Temparature) {
    println!("{:?} degrees F", temp.degrees_f);
}*/

impl Temperature {

    // Self returns 'Temperature'
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    // use reference &self, instead of temp::Temperature
    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }
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

    // variable 'book' is owned by 'main' function
    let book = Book {
        pages: 5,
        rating: 9,
    };

    // adding '&' to the argument to lend 'book' to functions 'display_page_count' and 'display_rating'
    // lending the reference of 'book' won't give ownership to the functions, and won't allow them to delete the variable 'book' after use
    display_page_count(&book);
    display_rating(&book);

    // implementing functions
    let hot = Temperature { degrees_f: 99.9 };

    /* Original function, before changing to impl
    Temperature::show_temp(hot);
    */

    // call function of Temperature type
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

}
