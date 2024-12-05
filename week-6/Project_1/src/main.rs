use std::io;

fn main() {
    loop {
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let choice: i32 = get_input("Enter your choice (1-6): ").parse().unwrap_or(0);

        match choice {
            1 => {
                let height: f64 = get_input("Enter the height: ").parse().unwrap();
                let base1: f64 = get_input("Enter the first base: ").parse().unwrap();
                let base2: f64 = get_input("Enter the second base: ").parse().unwrap();

                println!(
                    "Area of Trapezium: {:.2}",
                    area_of_trapezium(height, base1, base2)
                );
            }
            2 => {
                let diagonal1: f64 = get_input("Enter the first diagonal: ").parse().unwrap();
                let diagonal2: f64 = get_input("Enter the second diagonal: ").parse().unwrap();

                println!(
                    "Area of Rhombus: {:.2}",
                    area_of_rhombus(diagonal1, diagonal2)
                );
            }
            3 => {
                let base: f64 = get_input("Enter the base: ").parse().unwrap();
                let altitude: f64 = get_input("Enter the altitude: ").parse().unwrap();

                println!(
                    "Area of Parallelogram: {:.2}",
                    area_of_parallelogram(base, altitude)
                );
            }
            4 => {
                let side: f64 = get_input("Enter the side length: ").parse().unwrap();

                println!("Area of Cube: {:.2}", area_of_cube(side));
            }
            5 => {
                let radius: f64 = get_input("Enter the radius: ").parse().unwrap();
                let height: f64 = get_input("Enter the height: ").parse().unwrap();

                println!("Volume of Cylinder: {:.2}", volume_of_cylinder(radius, height));
            }
            6 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }

        println!();
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side * side
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}
