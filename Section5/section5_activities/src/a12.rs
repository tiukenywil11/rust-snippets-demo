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


struct ShippingBox{
    dimensions: i32,
    weight: i32,
    color: Color
}

impl ShippingBox {

    fn createNewBox() -> Self {
        Self {
            dimensions: 10,
            weight: 10,
            color: Color::Red
        }
    } 

    fn showBox(&self) {

        let mut temp = "";

        match self.color {
            Color::Red => temp = "red",
            Color::Blue => temp = "blue",
            Color::Yellow => temp = "yellow",
            _=> println!("color not found")
        }

        println!{"{:?}", self.dimensions};
        println!{"{:?}", self.weight};
        println!{"{:?}", temp};
    }

}

pub fn getBox() {

    let newBox = ShippingBox::createNewBox();
    newBox.showBox();

}
