// https://www.acmicpc.net/problem/2231

use std::io;

fn digit_gen(mut digit: i32) -> i32 {
    let mut sum: i32 = 0;

    while digit > 0 {
        sum += digit%10;
        digit /= 10;
    }

    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut d: i32 = 1;

    loop {
        if d + digit_gen(d) == n {
            break;
        }

        d += 1;

        if d > n {
            d = 0;
            break;
        }
    }

    println!("{}", d);
}