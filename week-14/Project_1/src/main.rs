use std::fs::File;
use std::io::{Read};
use std::io;

fn read_database_structure() -> String {
    read_table("globacom_dbase.sql")
}

fn read_project_table() -> String {
    read_table("project_tb.sql")
}

fn read_staff_table() -> String {
    read_table("staff_tb.sql")
}

fn read_customer_table() -> String {
    read_table("customer_tb.sql")
}

fn read_dataplan_table() -> String {
    read_table("dataplan_tb.sql")
}

fn read_table(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() {
    let mut user_role = String::new();

    println!("Welcome to Globacom Ltd");
    println!("Enter your role (administrator, project manager, employee, customer, vendor):");
    io::stdin().read_line(&mut user_role).unwrap();
    let user_role = user_role.trim();

    if user_role == "administrator" {
        println!("{}", read_database_structure());
    } 
    else if user_role == "project manager" {
        println!("{}", read_project_table());
    } 
    else if user_role == "employee" {
        println!("{}", read_staff_table());
    } 
    else if user_role == "customer" {
        println!("{}", read_customer_table());
    } 
    else if user_role == "vendor" {
        println!("{}", read_dataplan_table());
    } 
    else {
        println!("Invalid role. Please enter a valid user role.");
    }
}
