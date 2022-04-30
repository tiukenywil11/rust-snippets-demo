// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

fn is_restricted(customer_age:i32) -> Result<bool, String> {
    if customer_age >= 21 {
        return Ok(true);
    } else {
        return Err("Age must be above 20".to_owned());
    };
}

pub fn result() {

    let kenny = Customer {
        age: 27,
    };

    let restricted = is_restricted(kenny.age);

    println!("{:?}", restricted);

}
