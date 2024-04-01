// https://www.acmicpc.net/problem/9093

use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().to_owned()
}

fn main() {
    let t = r().parse::<usize>().unwrap();
    let mut output = BufWriter::new(stdout().lock());
    for i in 0..t {
        let strs: Vec<String> = r().split_whitespace().flat_map(str::parse).collect();
        for s in strs {
            let mut word: Vec<char> = s.chars().collect();
            word.reverse();
            let rev: String = word.into_iter().collect();
            write!(output, "{} ", rev);
        }
        writeln!(output, "");
    }
}