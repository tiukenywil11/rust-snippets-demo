fn main() {

    let integer = 2;
    let string = "hello";
    let character = 'j';
    let float = 0.5;
    let mut mutable_string = "Bill";
    let boolean = false;
    let variable = float;

    println!("Variables: {:?} {:?} {:?} {:?} {:?} {:?} {:?}", integer, string, character, float, mutable_string, boolean, variable);

    let add_x = add (1, 1);
    let add_y = add (3, 0);
    let add_z = add (add_x, 1);

    println!("Function calls: {:?} {:?} {:?}", add_x, add_y, add_z);

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
    
    println!("Hello, world!");

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

