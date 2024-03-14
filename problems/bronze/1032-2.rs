// https://www.acmicpc.net/problem/1032

use std::io;

fn read() -> String {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    n.trim_end().to_owned()
}


fn main() {
    let n: i32 = read().parse().unwrap();
    let mut pattern: Vec<char> = read().chars().collect();

    for _ in 0..n-1 {
        for (i, c) in read().chars().enumerate() {
            if pattern[i] != c {
                pattern[i] = '?'
            }
        }
    }

    println!("{}", pattern.iter().collect::<String>());
}