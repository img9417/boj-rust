// https://www.acmicpc.net/problem/1764

use std::io::{self, BufRead, Write, BufWriter};
use std::convert::TryInto;
use std::collections::HashMap;

fn r() -> String {
    let input = io::stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().to_owned()
}
fn main() {
    let [n, m]: [usize; 2] = r()
        .split_whitespace()
        .flat_map(str::parse)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
 
    let mut deutbo: HashMap<String, bool> = HashMap::new();

    for _ in 0..(n+m) {
        let name = r();
        match deutbo.get(&name) {
            Some(b) => deutbo.insert(name, !b),
            None => deutbo.insert(name, false)
        };
    }

    let mut deutbo: Vec<_> = deutbo
        .iter()
        .filter(|(_key, value)| **value)
        .map(|(key, _value)| key)
        .collect();
        
    deutbo.sort();

    let mut output = BufWriter::new(io::stdout().lock());
    writeln!(output, "{}", deutbo.len()).unwrap();
    for name in deutbo {
        writeln!(output, "{name}").unwrap();
    }
}