// this is the entry point of the application.
fn main() {

    // data type variables
    let integer = 2;
    let string = "hello";
    let character = 'j';
    let float = 0.5;
    let mut mutable_string = "Bill";
    let boolean = false;
    let variable = float;

    println!("Variables: {:?} {:?} {:?} {:?} {:?} {:?} {:?}", integer, string, character, float, mutable_string, boolean, variable);

    // function calls
    let add_x = add (1, 1);
    let add_y = add (3, 0);
    let add_z = add (add_x, 1);

    println!("Function calls: {:?} {:?} {:?}", add_x, add_y, add_z);

    // control flow statements
    let if_a = 99;

    println!("If: {:?}", if_a);
    
    if if_a > 99 {
        println!("{:?} is Big number", if_a);
    } else {
        println!("{:?} is Small number", if_a);
    }
    println!("Normal control flow");

    if if_a > 99 {
        if if_a > 200 {
            println!("{:?} is Huge number", if_a);
        } else {
            println!("{:?} is Big number", if_a);
        }
    } else {
        println!("{:?} is Small number", if_a);
    }
    println!("Nested control flow");

    // loop statements
    let mut loop_a = 0;

    loop {
        if loop_a == 5 {
            break;
        }
        println!("Loop: {:?}", loop_a);
        loop_a = loop_a + 1;
    }

    let mut while_a = 0;

    while while_a != 5 {
        println!("While: {:?}", while_a);
        while_a = while_a + 1;
    }
    
    // numeric types, and arithmetic
    let addition = 2 + 2;
    let subtraction = 10 - 5;
    let division = 10 / 2;
    let multiplication = 5 * 5;
    let remainder_division = 6 % 3;

    println!("Arithmetic values: {:?} {:?} {:?} {:?} {:?}", addition, subtraction, division, multiplication, remainder_division);

    println!("Hello, world!");

}

// add function, takes 2 32-bit integer parameters, and returns a 32-bit integer value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

