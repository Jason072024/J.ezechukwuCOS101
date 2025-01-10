use std::fs::File;
use std::io::Write;

fn main() {

    let drinks = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    let mut file = File::create("drinks.txt").expect("Failed to create file");

    for (category, items) in drinks {
        writeln!(file, "{}:", category).expect("Failed to write to file");
        for item in items {
            writeln!(file, "  - {}", item).expect("Failed to write to file");
        }
        writeln!(file).expect("Failed to write to file");
    }

    println!("Drink categories have been saved to 'drinks.txt'.");
}


