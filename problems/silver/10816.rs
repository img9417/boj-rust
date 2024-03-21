// https://www.acmicpc.net/problem/10816

use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, BufRead, BufWriter, Write},
};

trait Countable: Iterator {
    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = HashMap::new();
        self.for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
}

impl<T: Iterator> Countable for T {}


fn r() -> String {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().to_owned()
}

fn main() {
    r();
    let cards: HashMap<i32, usize> = r()
        .split_whitespace()
        .flat_map(str::parse::<i32>)
        .counts();
        // .into_iter()
        // .map(|(k, v)| (k as i32, v))
        // .collect();

    r();
    let v: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();

    let mut output = BufWriter::new(io::stdout().lock());
    for x in v {
        match cards.get(&x) {
            Some(cnt) => write!(output, "{cnt} ").unwrap(),
            None => write!(output, "0 ").unwrap(),
        };
    }
}
