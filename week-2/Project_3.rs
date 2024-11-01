fn main() {
    let p:f64 = 510000.0;
    let n:f64 = 3.0;
    let r:f64 = 5.0;

    let a = p * (1.0 - (r/100.0));
    println!("Amount is {}", a);
    let ci:f64 = a . powf(n);
    println!("Compound Interest is {}", ci);
    
}