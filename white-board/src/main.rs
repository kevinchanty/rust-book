use rand::Rng;
use std::io;

//Standard in, if expression
fn example1() {
    println!("Please choose true or false");
    let mut condition = String::new();
    io::stdin()
        .read_line(&mut condition)
        .expect("Failed to readline!");

    let condition = match condition.trim() {
        "true" => true,
        "false" => false,
        _ => panic!("Please insert boolean!"),
    };

    let result = if condition { 5 } else { 6 };

    println!("{result}");
}

fn main() {
    for number in (1..4).step_by(2) {
        println!("KKisnot {number}")
    }

    println!("kkisnot 21!")
}
