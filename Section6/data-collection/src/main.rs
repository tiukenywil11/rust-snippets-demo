// a structure test, with score data
struct Test {
    score: i32
}

// string
// strings are automatically borrowed
fn print_it(data: &str) {
    println!("{:?}", data);
}

struct Employee {
    name: String,
}

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {

    // vec macro, is similar to the code snipper below [vector manual]
    let my_numbers = vec![1,2,3];

    // [vector manual]
    let mut my_numbers = Vec::new();
    // adds '1', '2', & '3' to the vector
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    
    // remove number '3' from the vector
    my_numbers.pop();
    // returns '2' because there are only 2 items in the vector
    my_numbers.len();

    // gets the second index of my_numbers
    let two = my_numbers[1];

    // iterate the items in the vector

    // for is a loop, that's specific for collections
    for num in my_numbers {
        println!("{:?}", num);
    }

    // create a vector with Test struct data
    let my_scores = vec![
        Test {score: 90},
        Test {score: 88},
        Test {score: 77},
        Test {score: 93},
    ];

    for test in my_scores {
        println!{"score = {:?}", test.score};
    }

    // strings

    // automatically borrowed string '&str'
    print_it("a string slice");
    // owned string examples
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");

    // pass borrowed string, because that's what print_it expects
    print_it(&owned_string);
    print_it(&another_owned);

    // call Employee struct, needs to initialized an owned string before assigning
    // this allows struct Employee to delete the data of String after the program ends
    let emp_name = "Jayson".to_owned();
    let emp_name = String::from("Jayson");
    let emp = Employee {
        name: emp_name
    };

    let receipt = vec![
        LineItem{
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem{
            name: String::from("fruit"),
            count: 3,
        }
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count:{:?}", item.count);
    }

}
