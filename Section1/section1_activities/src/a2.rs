// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

pub fn add(value1: i32, value2: i32) -> i32 {
    let sum = value1 + value2;
    println!("{:?} + {:?} = {:?}", value1, value2, sum);
    return sum;
}