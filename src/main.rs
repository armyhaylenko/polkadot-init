use std::collections::HashMap;

#[derive(Default, Debug)]
struct Shop {
    storage: HashMap<String, (u8, u32)>,
}

impl Shop {
    pub fn new(products: Vec<(String, u8, u32)>) -> Self {
        let mut storage = HashMap::new();
        for (product_name, quantity, price) in products {
            storage.insert(product_name, (quantity, price));
        }
        Self { storage }
    }

    pub fn buy(
        &mut self,
        client: &mut Client,
        product_name: String,
        quantity: u8,
    ) -> Result<(), String> {
        let (available_quantity, price) = *self
            .storage
            .get(&product_name)
            .ok_or(String::from("No product of this name!"))?;
        if quantity > available_quantity {
            return Err(String::from(format!(
                "Cannot buy more than available! Available: {}.",
                available_quantity
            )));
        }

        client.add_product((product_name, quantity, price));

        Ok(())
    }

    pub fn checkout(&mut self, client: &mut Client) -> Result<(), String> {
        let cart = client.shopping_cart();
        let cart_sum = cart.iter().fold(0u32, |prev, (_, quantity, price)| {
            prev + (*quantity as u32) * (*price)
        });
        if cart_sum > client.balance {
            return Err(String::from("Insufficient balance!"));
        }
        client.balance -= cart_sum;

        for (name, quantity, _) in client.shopping_cart() {
            let (existing_product_quantity, price) = *self
                .storage
                .get(name)
                .ok_or(String::from("No such product!"))?;
            self.storage
                .insert(name.clone(), (existing_product_quantity - quantity, price))
                .unwrap(); // values must exist at the point of checkout
        }

        client.clear_cart();

        Ok(())
    }
}

struct Client {
    pub balance: u32,
    shopping_cart: Vec<(String, u8, u32)>,
}

impl Client {
    pub fn new(balance: u32) -> Self {
        Client {
            balance,
            shopping_cart: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: (String, u8, u32)) {
        self.shopping_cart.push(product)
    }

    pub fn shopping_cart(&self) -> &Vec<(String, u8, u32)> {
        &self.shopping_cart
    }

    pub fn clear_cart(&mut self) {
        self.shopping_cart.clear();
    }
}

fn main() {
    let mut shop = Shop::new(vec![
        ("milk".to_string(), 10, 1500),
        ("cereal".to_string(), 10, 2500),
    ]);
    println!("Our shop opened!: {:?}", &shop);
    let mut client = Client::new(10000);
    shop.buy(&mut client, "milk".to_string(), 5).unwrap();
    shop.buy(&mut client, "cereal".to_string(), 1).unwrap();
    println!(
        "Client's card after putting products there: {:?}",
        &client.shopping_cart()
    );
    shop.checkout(&mut client).unwrap();
    println!(
        "Client's card after the checkout: {:?}",
        &client.shopping_cart()
    );

    assert_eq!(client.balance, 0);
    assert_eq!(client.shopping_cart.len(), 0);

    println!("The shop closes!: {:?}", &shop);
}
