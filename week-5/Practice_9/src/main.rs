fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("The value of A: {}",a);
    println!("The value of B: {}",b);

    let mut res = a > b;
    println!("A greater than B: {}",res);

    res = a < b;
    println!("A lesser than B: {}",res);

    res = a >= b;
    println!("A greater than or equals to B: {}",res);

    res = a <= b;
    println!("A lesser than or equals to B: {}",res);

    res = a == b;
    println!("A is equals to B: {}",res);

    res = a != b;
    println!("A is not equals to B: {}",res);
}
