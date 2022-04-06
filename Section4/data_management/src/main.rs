fn main() {

    // takes the argument Direction
    println!("{:?}", which_way(Direction::Left));

}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn which_way<'life>(go: Direction) -> &'life str{

    let mut temp = "";

    match go {
        Direction::Up => temp = "go up",
        Direction::Down => temp = "go down",
        Direction::Left => temp = "go left",
        Direction::Right => temp = "go right",
    };

    return temp;
}