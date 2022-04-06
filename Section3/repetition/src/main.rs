fn main() {

    let mut loop_i = 3;

    loop {
        println!("{:?}", loop_i);
        loop_i = loop_i - 1;
        if loop_i == 0 {
            break;
        }
    }
    println!("done with loop!");

    let mut while_i = 1;

    while while_i <= 3 {
        println!("{:?}", while_i);
        while_i = while_i + 1;
    }
    println!("done with while!");

}
