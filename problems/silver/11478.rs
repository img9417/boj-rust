// https://www.acmicpc.net/problem/11478

use std::io::{stdin, stdout, BufWriter, BufRead, Write};
use std::collections::HashSet;

fn main() {
    let input = stdin().lock();
    let mut line = input.lines();
    let s: String = line.next().unwrap().unwrap().trim().to_owned();
    let mut set: HashSet<&str> = HashSet::new();

    let mut l = 1;
    while l <= s.len() {
        let mut low = 0;
        let mut high = low+l;

        while high <= s.len() {

            let part: &str = &s[low..high];
            set.insert(part);

            low += 1;
            high += 1;
        }

        l += 1;
    }

    let mut output = BufWriter::new(stdout().lock());
    writeln!(output, "{}", set.len()).unwrap();
}