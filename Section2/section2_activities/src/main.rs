mod a3a;
mod a3b;
mod a4a;
mod a4b;

fn main() {
    // call function from a3a.rs, takes argument boolean
    a3a::displayGreeting(true);
    // call function from a3b.rs, takes argument i32
    a3b::dsiplayAmount(1);
    // call function from a4a.rs, takes argument boolean
    a4a::displayBoolean(true);
    // call function from a4a.rs, takes argument i32
    a4b::displayNumber(1);
}
