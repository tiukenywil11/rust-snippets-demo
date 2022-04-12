mod a10;
mod a11;
mod a12;

fn main() {
    
    let grocery_item = a11::GroceryItem {
        id: 1,
        quantity: 1,
    } ;

    // call function from a10.rs, takes argument i32
    a10::valueExpression(1);
    // call function from a11.rs, takes argument GroceryItem
    a11::groceryList(grocery_item);
    // call function from a12.rs, takes no argument
    a12::getBox();

}
