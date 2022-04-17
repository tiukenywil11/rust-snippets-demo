// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct FavColor {
    age: i32,
    name: String,
    color: String,
}

pub fn favoriteColor() {

    let persons = vec![
        FavColor {
            age: 27,
            name: "Kenny".to_owned(),
            color: "orange".to_owned(),
        },
        FavColor {
            age: 20,
            name: "user2".to_owned(),
            color: "black".to_owned(),
        },
        FavColor {
            age: 30,
            name: "user3".to_owned(),
            color: "white".to_owned(),
        },
    ];

    for person in persons {
        if(person.name == "Kenny".to_owned()) {
            println!("age: {:?}, name: {:?}, color: {:?}", person.age, person.name, person.color);
        }
    }
}
