use std::io;

fn main() {
    
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    
    println!("Enter number for a:");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Please input a valid number for a");

    
    println!("Enter number for b:");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Please input a valid number for b");


    println!("Enter number for c:");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Please input a valid number for c");

    
    let d = b * b - 4.0 * a * c;
    println!("The value of d is {}",d);

    
    if d > 0.0 {
        println!("There will be two distinct roots.");
    } else if d == 0.0 {
        println!("There will be exactly one root.");
    } else {
        println!("There will be no real roots.");
    }

    
    if d >= 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("The roots of the quadratic equation are: {} and {}", root1, root2);
    }
}
