// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

pub enum Color {
    Red,
    Blue,
    Yellow,
}

pub fn colorPicker<'life>(color: Color) -> &'life str {

    let mut temp = "";

    match color {
        Color::Red => temp = "red",
        Color::Blue => temp = "blue",
        Color::Yellow => temp = "yellow",
    };

    println!("{:?}", temp);
    return temp;
}
