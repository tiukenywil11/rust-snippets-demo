// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

pub fn valueExpression<'life>(value:i32) -> &'life str {

    let mut temp = "";

    let is_gt_100 = value > 100;

    match is_gt_100 {
        true => temp = "its big",
        false => temp = "its small"
    };

    println!("{:?}", temp);

    return temp;

}
