// https://www.acmicpc.net/problem/1436

use std::io;

fn contains666(n: u64) -> bool {
    // let n: Vec<char> = n.to_string().chars().collect();

    // for i in 0..n.len()-2 {
    //     if n[i]=='6' && n[i+1] == '6' && n[i+2] == '6' {
    //         return true
    //     }
    // }

    let mut n = n;
    
    while n > 0 {
        if n % 1000 == 666 {
            return true
        }
        n /= 10;
    }

    false
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse().unwrap();

    let mut nums: Vec<u64> = vec![666, 1666, 2666, 3666, 4666, 5666, 6660];
    let mut target = 6661;

    while nums.len() < n {
        if contains666(target) {
            nums.push(target);
        }
        target += 1;
    }

    println!("{}", nums[n-1]);
}