use std::io;

mod quick_swap;
/// .
fn main() {
    let mut int1: String = String::new();
    let mut int2: String = String::new();

    println!("Enter Two Unsigned 32-bit Integers:");

    io::stdin()
        .read_line(&mut int1)
        .expect("WRONG");

    let int1: u32 = match int1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("AAAAAAAAAAAA NOT AN UNSIGNED 32-BIT INTEGER!!!!!!!!!"),
    };

    io::stdin()
        .read_line(&mut int2)
        .expect("WRONG");

    let int2: u32 = match int2.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("AAAAAAAAAAAA NOT AN UNSIGNED 32-BIT INTEGER!!!!!!!!!"),
    };

    let mut a = [int1, int2];
    a = quick_swap::quick_swap(a);
    println!("{:?}",a);
}
