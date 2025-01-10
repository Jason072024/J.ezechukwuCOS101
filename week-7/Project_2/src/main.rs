use std::io;

fn main() {

    println!("Enter the number of interviews: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: usize = input.trim().parse().expect("Invalid input");

    let mut participants: Vec<(String, u32)> = Vec::new();

    for _ in 0..number {

        println!("Enter the name of the participant: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let name = input1.trim().to_string();

        println!("Enter their years of experience: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let year: u32 = input2.trim().parse().expect("Invalid input");

        participants.push((name, year));
    }

    println!("\nList of Participants:");
    for (i, (name, year)) in participants.iter().enumerate() {
        println!("{}. Name: {}, Years of Experience: {}", i + 1, name, year);
    }
}
