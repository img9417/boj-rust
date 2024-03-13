use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x: u8 = input.trim().parse().unwrap();

    match x {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        _ => println!("F"),
    }
}