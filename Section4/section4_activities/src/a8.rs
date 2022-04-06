// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

pub enum Flavor {
    Orange,
    Apple,
    Grapes
}

pub struct Drink {
    pub flavor: Flavor,
    pub fluid_ounce: i32, 
}

pub fn getDrink<'life>(drink:Drink) -> &'life str {

    let mut temp = "";
    
    match drink.flavor {
        Flavor::Orange => temp = "orange",
        Flavor::Apple => temp = "apple",
        Flavor::Grapes => temp = "grape",
    }

    println!("{:?}, {:?}", temp, drink.fluid_ounce);

    println!("{:?}", temp);
    return temp;

}
