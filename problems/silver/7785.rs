// https://www.acmicpc.net/problem/7785

use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let input = stdin.lock();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut people: HashSet<String> = HashSet::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_owned();
        let action = parts.next().unwrap();

        if action == "enter" {
            people.insert(name);
        } else {
            people.remove(&name);
        }
    }

    let mut ppl: Vec<_> = people.into_iter().collect();
    ppl.sort_by(|a, b| b.cmp(a));

    let mut output = BufWriter::new(stdout().lock());
    for x in ppl {
        writeln!(output, "{x}").unwrap();
    }
}
