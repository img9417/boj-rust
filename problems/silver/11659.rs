// https://www.acmicpc.net/problem/11659

use std::io::{self, BufRead, Write, BufWriter};

// fn r() -> Vec<usize> {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     input.split_whitespace().flat_map(str::parse).collect()
// }

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let input = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());
    
    let mut lines = input.lines();

    let nm: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().flat_map(str::parse).collect();
    let nums: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().flat_map(str::parse).collect();
    let mut s: Vec<usize> = vec![0; nm[0]+1];

    for i in 1..=nm[0] {
        s[i] = s[i-1] + nums[i-1];
    }

    for line in lines.take(nm[1]) {
        let xy: Vec<usize> = line.unwrap().split_whitespace().flat_map(str::parse).collect();
        writeln!(output, "{}", s[xy[1]]-s[xy[0]-1]).unwrap();
    }

    // for _ in 0..nm[1] {
    //     let xy = r();
    //     println!("{}", s[xy[1]] - s[xy[0]-1]);
    // }
}