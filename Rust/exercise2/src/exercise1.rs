struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("Id: {}", item.id);
}

fn main() {
    let item: GroceryItem = GroceryItem { id: 0, quantity: 5 };
    display_id(&item);
    display_quantity(&item);
}
