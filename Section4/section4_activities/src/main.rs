mod a7;
mod a8;

fn main() {

    let orange_drink = a8::Drink {
        flavor: a8::Flavor::Orange,
        fluid_ounce: 12,
    };

    // call function from a7.rs, takes argument Color
    a7::colorPicker(a7::Color::Red);
    // call function from a8.rs, takes argument Flavor
    a8::getDrink(orange_drink);

}
