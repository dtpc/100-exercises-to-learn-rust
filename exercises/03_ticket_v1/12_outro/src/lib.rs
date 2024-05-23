// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.


pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {
    fn validate_name(name: &String) {
        if name.is_empty() {
            panic!("Product name cannot be empty")
        }
        if name.len() > 300 {
            panic!("Product name cannot be more than 300 characters")
        }
    }
    fn validate_quantity(quantity: &u32) {
        if *quantity == 0 {
            panic!("Quantity must be greater than zero")
        }
    }
    fn validate_price(price: &u32) {
        if *price == 0 {
            panic!("Price must be greater than zero")
        }
    }

    pub fn new(name: String, quantity: u32, price: u32) -> Self {
        Order::validate_name(&name);
        Order::validate_quantity(&quantity);
        Order::validate_price(&price);
        Order {
            product_name: name, 
            quantity: quantity, 
            unit_price: price
        }
    }
    
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn set_product_name(&mut self, name: String) {
        Order::validate_name(&name);
        self.product_name = name;
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn set_quantity(&mut self, quantity: u32) {
        Order::validate_quantity(&quantity);
        self.quantity = quantity;
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
    pub fn set_unit_price(&mut self, price: u32) {
        Order::validate_price(&price);
        self.unit_price = price;
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

}