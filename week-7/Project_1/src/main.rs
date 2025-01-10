fn main() {
    
    let input = vec!["Office Administrator", "Accademic", "Lawyer", "Teacher"];

    let mut input1 = String::new();
    println!("Enter the occupation: ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let occupation = input.trim().to_string();

    for occupation in input {
        if occupation == "Office Administrator"
        {

        }
    }
}
