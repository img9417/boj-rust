// https://www.acmicpc.net/problem/5585

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    
    let coin = [500, 100, 50, 10, 5, 1];
    let mut otsuri = 1000-n;
    let mut coins = 0;

    for c in coin {
        coins += otsuri/c;
        otsuri %= c;
    }

    println!("{}", coins);
}