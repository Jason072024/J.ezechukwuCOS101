struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    let quantity = 3; // Number of laptops purchased from each brand

    let total_cost = hp.total_cost(quantity)
        + ibm.total_cost(quantity)
        + toshiba.total_cost(quantity)
        + dell.total_cost(quantity);

    println!("The total cost for purchasing {} laptops from each brand is: ₦{}", quantity, total_cost);
}

