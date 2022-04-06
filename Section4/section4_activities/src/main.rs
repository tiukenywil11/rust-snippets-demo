mod a7;
mod a8;

fn main() {

    // call function from a7.rs, takes argument Color
    a7::colorPicker(a7::Color::Red);
    // call function from a8.rs, takes no argument
    a8::getDrink();

}
