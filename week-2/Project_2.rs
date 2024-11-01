fn main() {
    let t:f64 = 450000.0;
    let m:f64 = 1500000.0;
    let h:f64 = 750000.0;
    let d:f64 = 2850000.0;
    let a:f64 = 250000.0;

    let s = (2.0 * t) + (1.0 * m) + (3.0 * h) + (3.0 * d) + (1.0 * a);
    println!("Sum is {}", s);
    let v = s/5.0;
    println!("Average is {}", v);
    
}