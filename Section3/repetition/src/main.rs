fn main() {

    let mut loop_i = 3;

    loop {
        println!("{:?}", loop_i);
        loop_i = loop_i - 1;
        if loop_i == 0 {
            break;
        }
    }
    println!("done!");

}
