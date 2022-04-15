// a structure test, with score data
struct Test {
    score: i32
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

    // create a vector with Test struct datas
    let my_scores = vec![
        Test {score: 90},
        Test {score: 88},
        Test {score: 77},
        Test {score: 93},
    ];

    for test in my_scores {
        println!{"score = {:?}", test.score};
    }

}
