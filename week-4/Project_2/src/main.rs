use std::io;

fn main() {
    let mut level = String::new();  
    let mut input2 = String::new(); 

    println!("Enter the level of the Employee (experienced/inexperienced):");
    io::stdin().read_line(&mut level).expect("Failed to read line");
    let level = level.trim(); 

    println!("Enter the Employee's age:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let age: i32 = input2.trim().parse().expect("Please enter a valid number for age");

    if level == "experienced" && age >= 40 
    {
        println!("The employee's Incentive is N1,560,000");
    } 
    else if level == "experienced" && age >= 30 && age < 40 
    {
        println!("The employee's Incentive is N1,480,000");
    } 
    else if level == "experienced" && age < 28 
    {
        println!("The employee's Incentive is N1,300,000");
    } 
    else if level == "inexperienced" && age >= 28 && age < 40 
    {
        println!("The employee's Incentive is N100,000");
    } 
    else if level == "inexperienced" && age < 28 
    {
        println!("The employee's Incentive is N100,000");
    }  
    else 
    {
        println!("Invalid input or incentive not defined for this combination.");
    }    
   
    
}
