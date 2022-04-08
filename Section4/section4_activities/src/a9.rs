// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

pub fn getCoordinates(coordinate:(i32, i32)) -> (i32, i32) {
    let x = coordinate.0;
    let y = coordinate.1;

    let mut temp = "";

    if y > 5 {
        temp = "y coordinate > 5";
    } else if y < 5 {
        temp = "y coordinate < 5";
    } else {
        temp = "y coordinate = 5";
    }

    println!("{:?}", temp);
    return (x, y);
}
