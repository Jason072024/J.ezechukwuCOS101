use std::io;

fn main() {
    let mut menu = String::new();
    let mut quantity = String::new();

    println!("           MENU                        PRICE");
    println!("p = Pounded yam / Edinkaiko Soup       N3200");
    println!("f = Fried Rice & Chicken               N3000");
    println!("a = Amala & Ewedu Soup                 N2500");
    println!("e = Eba & Egusi Soup                   N2000");
    println!("w = White Rice & Stew                  N2500");

    println!("Enter your menu: ");
    io::stdin().read_line(&mut menu).expect("Failed to read input");
    let menu = menu.trim();

    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: f32 = quantity.trim().parse().expect("Failed to input the quantity");

    let price: f32;

    price = match menu {
        "p" => 3200.0,
        "f" => 3000.0,
        "a" => 2500.0,
        "e" => 2000.0,
        "w" => 2500.0,
        _ => {
            println!("Invalid selection of menu.");
            return;
        }
    };

    let total_price = price * quantity;

    if total_price > 10000.00 
    {
        let total = total_price - (total_price * 0.5);
        println!("Total cost for {} item(s) of {} is: N{:.2}", quantity, menu, total);
    } 
    else 
    {
        println!("Total cost for {} item(s) of {} is: N{:.2}", quantity, menu, total_price);
    }
}
