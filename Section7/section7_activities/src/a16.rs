// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

#[derive(Debug)]
struct StudentInfo {
    name: String,
    locker: Option<i32>
}

pub fn option() {

    let student_info = StudentInfo{
        name: "Kenny".to_owned(),
        locker: None
    };

    println!("{:?}", student_info);

}
