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

struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {

    // takes the argument Direction
    println!("{:?}", which_way(Direction::Left));

    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    let tall = my_box.height;
    println!("the box is {:?} units tall", tall);

}