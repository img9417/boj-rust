// https://www.acmicpc.net/problem/10809

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut freq: Vec<i32> = vec![-1; 26];

    let mut i = 0;
    for ch in input.chars() {
        let index = ((ch as i32) - ('a' as i32)) as usize;
        if freq[index] == -1 {
            freq[index] = i; 
        }
        i += 1;
    }

    for f in freq {
        print!{"{} ", f};
    }
}