// https://www.acmicpc.net/problem/10809

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    ('a'..='z').for_each(|ch| {
        if let Some(idx) = input.find(ch) {
            print!{"{idx} "};
        } else {
            print!("-1 ");
        }
    })
}