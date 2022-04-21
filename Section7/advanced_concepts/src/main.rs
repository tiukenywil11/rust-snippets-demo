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
}
