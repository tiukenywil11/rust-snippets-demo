// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

pub struct GroceryItem {
    pub id: i32,
    pub quantity: i32,
}

fn display_id(grocery_item: &GroceryItem) {
    println!("pages = {:?}", grocery_item.id);
}

fn display_quantity(grocery_item: &GroceryItem) {
    println!("rating = {:?}", grocery_item.quantity);
}

pub fn groceryList(grocery_item:GroceryItem) {

    display_id(&grocery_item);
    display_quantity(&grocery_item);

}
