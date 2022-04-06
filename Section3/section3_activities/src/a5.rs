// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

pub fn incrementTo4(count:i32) {

    let mut temp = count;

    loop {
        if temp > 4 {
            println!("{:?} is greater than 4", temp);
            break;
        }
        println!("{:?}", temp);
        temp = temp + 1;
    }

}
