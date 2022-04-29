// deriving functions

// derive a macro Debug that applies to enum and structs
// implement debug printing for both position and Employee 
// derive macros Clone and Copy that applies to enum and structs
// implement clone and copy everytime Position, or Employee is called, this creates a new copy, and helps with borrowing 
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

// need to add the same derive macro for enum 'Position', since enum 'Position' is used here
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

// type annotations

fn print_many(msg: &str, count: i32) {}

enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    // advanced enum
    // new variants with additional data
    Scroll(i32),
    Move(i32, i32),
}

// advanced enum

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

// advance match
struct Ticket { 
    event: String,
    price: i32,
}

// option: optional data

/*
-- T is a placeholder type, can be anything
enum Option<T> {
    -- T is whatever type specified by the program
    Some(T),
    None
}
*/

struct Customer {
    age: Option<i32>,
    email: String,
}

struct GroceryItem {
    name: String,
    qty: i32,
}

// returns optional data
fn find_quantity(name: &str) -> Option<i32> {
    
    let groceries = vec![
        GroceryItem{ name: "bananas".to_owned(), qty: 4, },
        GroceryItem{ name: "eggs".to_owned(), qty: 12, },
        GroceryItem{ name: "bread".to_owned(), qty: 1, },
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }
    // else returns 'None'
    None
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

// result

/*
-- result is an enum with 2 variance, Ok with parameter T (any type), and Error with parameter E (any type for error)
enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/

/*
-- function get sound returns sound data if successful, and returns a string if unsuccessful
-- SoundData is a fictional data, so this would cause an error
fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert")),
    } else {
        Err("unable to find sound data".to_owned())
    }
}
*/

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

// question mark operator
// returns nothing "()" unit type ", or error
fn pick_choice(input: &str) -> Result<(), String> {
    // question mark will automatically perform a match operation
    // if successful, will take the inner data, else will return error
    let choice = get_choice(input)?;
    print_choice(&choice);
    // return nothing
    Ok(())
}

/// This is a documentation comment
fn main() {

    let me = Employee {
        position: Position::Worker,
        work_hours: 40
    };

    /* this is replaced by the debug macro
    match me.position {
        Position::Manager => println!("manager"),
        Position::Supervisor => println!("supervisor"),
        Position::Worker => println!("worker"),
    };
    */

    // this gets Debug for Position
    println!("{:?}", me.position);

    // this gets Debug for Employee
    println!("{:?}", me);

    // with Clone and Copy macros derived to struct "Employee", and enum "Position". Using "me" without borrowing will not cause an error, since a new copy is passed.
    print_employee(me);
    print_employee(me);

    // type annotations

    let num: i32 = 15;
    let a: char = 'a';
    let left_click: Mouse = Mouse::LeftClick;

    // generics type annotations: Vectors
    // specify the type of vectors, recommended when using struct / enum type
    let numbers: Vec<i32> = vec![1,2,3];
    let letters: Vec<char> = vec!['a', 'b'];
    let clicks: Vec<Mouse> = vec![
        Mouse::LeftClick,
        Mouse::LeftClick,
        Mouse::RightClick,
    ];

    // advance match
    let n = 3;
    match n {
        3 => println!("three"),
        // this is replaced by "other"
        // _=> println!("number: {:?}", n)
        other => println!("number: {:?}", other)
    }

    // match variant with extra data
    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("flat 2"),
        // ignore any number on Flat discount
        // underscore replaced by 'amount'
        // Discount::Flat(_) =>
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        // ignore everything else
        _ => ()
    }

    // match struct
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50
    };

    match concert {
        Ticket {price: 50, event} => println!("event @50 = {:?}", event),
        // only match price whatever it may be in struct
        Ticket {price, ..} => println!("price = {:?}", price),
    }

    // option

    let mark = Customer {
        age: Some(22), email: "mark@example.com".to_owned(),
    };

    let becky = Customer {
        age: None, email: "becky@example.com".to_owned(),
    };

    match becky.age{
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age is not provided")
    }

    let quantity = find_quantity("bananas");

    println!("quantity: {:?}", quantity);

    let response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response")
    }

    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        None => println!("q2: no response")
    }

    match response.q3 {
        Some(ans) => println!("q3: {:?}", ans),
        None => println!("q3: no response")
    }

    // using rust standard library [https://doc.rust-lang.org/std/#modules]

    // use rust vector library [https://doc.rust-lang.org/std/vec/struct.Vec.html] 
    let numbers = vec![1, 2, 3];

    match numbers.is_empty() {
        true => println!("no numbers"),
        false => println!("has numbers")
    }

    // result
    
    /* 
    -- SoundData is a fictional data, so this would cause an error
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("sound data located"),
        Err(e) => println!("error: {:?}", e),
    }
    */

    let choice = get_choice("leave");

    //println!("choice = {:?}", choice);

    /*
    -- this will cause an error, because choice is a "Result" type, and print_choice needs a "MenuChoice" type
    print_choice(&choice);
    */

    // this will fix print_choice function
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }

    pick_choice("start");
}
