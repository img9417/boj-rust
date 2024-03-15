// https://www.acmicpc.net/problem/10101

use std::io;

fn r() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()    
}

fn main() {
    let a1 = r();
    let a2 = r();
    let a3 = r();
    let sum = a1+a2+a3;

    match (a1, a2, a3, sum) {
        (a1, a2, a3, sum) if sum==180 && a1 != a2 && a1 != a3 && a2 != a3 => println!("Scalene"),
        (a1, _, _, sum) if sum==180 && a1 == 60 => println!("Equilateral"),
        (a1, a2, a3, sum) if sum==180 && a1 == a2 || a1 == a3 || a2 == a3 => println!("Isosceles"),
        _ => println!("Error"),
    }
}