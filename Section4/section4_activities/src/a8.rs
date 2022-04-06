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


struct Drink {
    flavor: &'static str,
    fluid_ounce: i32, 
}

pub fn getDrink<'life>() -> &'life str {

    let mut flavor = "";

    let orange_drink = Drink {
        flavor: "orange",
        fluid_ounce: 12,
    };

    println!("{:?}, {:?}", orange_drink.flavor, orange_drink.fluid_ounce);
    
    match orange_drink.flavor {
        orange => flavor = "orange",
    }

    println!("{:?}", flavor);
    return flavor;

}
