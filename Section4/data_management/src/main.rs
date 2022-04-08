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

struct GroceryItem {
    stock: i32,
    price: f64,
}

enum Access {
    Full,
}

// create a tuple, surrounded by parethesis
fn one_two_three() -> (i32, i32, i32) {
    (1,2,3)
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

    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);

    let numbers = one_two_three();
    // destructure tuple, and put them in individual variables
    let (x, y, z) = one_two_three();
    println!("{:?} {:?}", x, numbers.0);
    println!("{:?} {:?}", y, numbers.1);
    println!("{:?} {:?}", z, numbers.2);

    // create a tuple, with an enum value
    let (employee, access) = ("Jake", Access::Full);

    let coord = (2, 3);
    println!("{:?} {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?} {:?}", x, y);

    let user_info = ("Emma", 20);
    let (name, age) = ("Emma", 20);

    // this should be replaced by structures
    // best practice: tuples should be limited to 3
    let favorites = ("red", 14, "TX", "pizza", "TV SHOW", "home");
    let state = favorites.2;
    let place = favorites.5;

}