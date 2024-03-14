// https://www.acmicpc.net/problem/1032

use std::io;
use std::iter::FromIterator;

fn read_int() -> usize {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    n.trim_end().parse().unwrap()
}
fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}

fn main() {
    let n = read_int();
    let mut filenames: Vec<String> = Vec::with_capacity(n);

    for _ in 0..n {
        filenames.push(read_str());
    }

    let mut pattern: Vec<char> = filenames[0].chars().collect();
    let mut files: Vec<Vec<char>> = vec![];

    for s in filenames {
        files.push(s.chars().collect());
    }

    let size = files.len();

    for i in 0..pattern.len() {
        for j in 0..size-1 {
            if files[j][i] != files[j+1][i] {
                pattern[i] = '?';
            }
        }
    }

    let pattern = String::from_iter(pattern);
    println!("{pattern}");
}