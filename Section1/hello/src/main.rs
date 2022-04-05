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

    println!("Hello, world!");

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

