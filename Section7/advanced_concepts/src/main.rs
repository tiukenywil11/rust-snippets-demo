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
}
