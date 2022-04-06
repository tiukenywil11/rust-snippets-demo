// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

pub fn decrement5To1(count:i32) {

    let mut temp = count;

    while temp >= 1 {

        if temp > 5 {
            println!("{:?} is greater than 5", temp);
            break;
        }

        println!("{:?}", temp);
        temp = temp - 1;
    }

}
