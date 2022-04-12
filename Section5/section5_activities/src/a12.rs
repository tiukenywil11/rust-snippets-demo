// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Blue,
    Yellow
}

impl Color {

    fn stringColor<'life>(&self) -> &'life str {

        let mut temp = "";

        match self {
            Color::Red => temp = "red",
            Color::Blue => temp = "blue",
            Color::Yellow => temp = "yellow",
            _=> println!("color not found")
        }

        return temp;

    }
}

struct Dimension {
    length: i32,
    width: i32,
    height: i32
}

impl Dimension {

    fn stringDimension(&self) {
        println!("length: {:?}", self.length);
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);

    }
}

struct ShippingBox{
    dimension: Dimension,
    weight: i32,
    color: Color
}

impl ShippingBox {

    // automatically gets values from parameters, Color and Dimension can be called, because it is part of struct ShippingBox
    fn createNewBox(weight: i32, color: Color, dimension: Dimension) -> Self {
        Self {
            dimension,
            weight,
            color
        }
    } 

    fn showBox(&self) {

        self.dimension.stringDimension();
        println!{"weight: {:?}", self.weight};
        println!{"color: {:?}", self.color.stringColor()};
    }

}

pub fn getBox() {

    let dimension = Dimension {
        length: 2,
        width: 2,
        height: 2
    };

    let newBox = ShippingBox::createNewBox(1, Color::Red, dimension);

    newBox.showBox();

}
