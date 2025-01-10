use std::fs::File;
use std::io::Write;

fn main() {

    let students = vec![
        ("John Doe", 20, "Computer Science"),
        ("Jane Smith", 19, "Mechanical Engineering"),
        ("Ali Bello", 21, "Physics"),
    ];

    let mut file = File::create("students.txt").expect("Failed to create file");

    writeln!(file, "Name, Age, Department").expect("Failed to write to file");
    for (name, age, department) in students {
        writeln!(file, "{}, {}, {}", name, age, department).expect("Failed to write to file");
    }

    println!("Student information has been saved to 'students.txt'.");
}
