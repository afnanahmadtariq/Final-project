use std::collections::HashMap;

#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn add_product(&mut self, product: Product) {
        self.products.insert(product.name.clone(), product);
        println!("Product added successfully");
    }

    fn edit_product(&mut self, name: &str, updated_product: Product) {
        if let Some(mut product) = self.products.get_mut(name) {
            product.name = updated_product.name;
            product.description = updated_product.description;
            product.price = updated_product.price;
            product.quantity = updated_product.quantity;
        } else {
            println!("Error: Product not found!");
        }
    }

    fn delete_product(&mut self, name: &str) {
        if self.products.remove(name).is_some() {
            println!("Product deleted successfully!");
        } else {
            println!("Error: Product not found!");
        }
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        println!("{:+}", "");
        println!(
                 "| Name         | Description                                                | Price    | Quantity |"
        );
        println!("|--------------|------------------------------------------------------------|----------|----------|");
        for product in self.products.values() {
            println!(
                "| {:12} | {:50} | {:.2} | {:>8} |",
                product.name, product.description, product.price, product.quantity
            );
        }
        println!("{:+}", "");
    }
}

fn main(){
    while true{
        println!("Select an option (Enter corespondig number):");
        println!("1) Log in");
        println!(" Press any buttom to exit program.");
        let mut choice  = String::new();
        std::io::stdin().read_line(&mut choice).expect("failed to readline");
        if let Ok(num) = choice.trim().parse::<i32>() {
            if num == 1{
                log_in();
            }
        }
        else{
            break;
        }
    }
    println!("Program exited!");
}

fn log_in() {
    println!("Enter username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("failed to read line");

    println!("Enter password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).expect("failed to read line");

    // Replace with proper authentication logic (avoid storing credentials in code)
    if username.trim() == "admin" && password.trim() == "pass123" {
        println!("Logged in successfully!");
        launch();
    } 
    else {
        println!("Invalid username or password.");
    }
}
fn launch(){
    let mut inventory: Inventory = Inventory {products : HashMap::new()};
    while true{
        println!("---Welcome to rusty inventory management system---");
        println!("---Select an option (Enter corespondig number)---");
        println!("1) Generate Report for current products in Inventory.");
        println!("2) Add a new item to the inventory.");
        println!("3) Edit existing item's information.");
        println!("4) Remove an item from the inventory.");
        println!(" Press any buttom to Log out.");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("failed to readline");
        if let Ok(num) = choice.trim().parse::<i32>(){
            match num{ 
                1=>inventory.generate_report(), 
                2=>add_product(&mut inventory), 
                3=>edit_product(&mut inventory), 
                4=>delete_product(&mut inventory), 
                _=>break,
            }
        };
    }
}
fn add_product(inventory: &mut Inventory){
    println!("Enter name of product:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("failed to readline");
    println!("Enter Description of product:");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("failed to readline");
    println!("Enter price of product:");
    let mut price = String::new();
    std::io::stdin().read_line(&mut price).expect("failed to read line");
    if let Ok(value) = price.trim().parse::<f64>() {
        println!("Enter quantity of product:");
        let mut quantity = String::new();
        std::io::stdin().read_line(&mut quantity).expect("failed to readline");
        if let Ok(quantity_value) = quantity.trim().parse::<i32>(){
            inventory.add_product(Product{name,description,price: value,quantity: quantity_value});
        }
        else{
            println!("Error: Input is not a valid price.");
        }
    }
    else{
        println!("Error: Input is not a valid price.");
    }
}
fn edit_product(inventory: &mut Inventory){
    println!("Enter name of product to edit:");
    let mut search_term = String::new();
    std::io::stdin().read_line(&mut search_term).expect("Failed to search by name.");
    println!("---Enter new info----");
    
    println!("Enter name of product:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("failed to readline");
    println!("Enter Description of product:");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("failed to readline");
    println!("Enter price of product:");
    let mut price = String::new();
    std::io::stdin().read_line(&mut price).expect("failed to read line");
    if let Ok(value) = price.trim().parse::<f64>() {
        println!("Enter quantity of product:");
        let mut quantity = String::new();
        std::io::stdin().read_line(&mut quantity).expect("failed to readline");
        if let Ok(quantity_value) = quantity.trim().parse::<i32>() {
            inventory.edit_product(&name.clone(), Product{name,description,price: value,quantity: quantity_value});
        }
        else{
            println!("Error: Input is not a valid price.");
        }
    }
    else{
        println!("Error: Input is not a valid price.");
    }
}
fn delete_product(inventory: &mut Inventory){
    println!("Enter name of product to delete:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("failed to readline");
    inventory.delete_product(&name);
}