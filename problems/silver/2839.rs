// https://www.acmicpc.net/problem/2839

use std::io;
use std::cmp::{min};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut dp: Vec<usize> = vec![5001; 5001];

    dp[3] = 1;
    dp[5] = 1;

    for i in 6..=n {
        dp[i] = min(dp[i-3], dp[i-5]) + 1;
    }

    if dp[n] >= 5001 {
        println!("-1");
    } else {
        println!("{}", dp[n]);
    }
}