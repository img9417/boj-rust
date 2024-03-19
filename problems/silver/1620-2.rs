// https://www.acmicpc.net/problem/1620

use std::io::{self, Write, BufWriter};
use std::collections::HashMap;

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main() {
    let nm: Vec<usize> = r().split_whitespace().flat_map(str::parse).collect();
    let mut name_index: HashMap<String, usize> = HashMap::with_capacity(nm[0]);
    let mut index_name: Vec<String> = Vec::with_capacity(nm[0]);

    for i in 1..=nm[0] {
        let pokemon = r();
        index_name.push(pokemon.clone());
        name_index.insert(pokemon, i);
    }

    let mut output = BufWriter::new(io::stdout().lock());
    for _ in 0..nm[1] {
        let input = r();

        if let Ok(i) = input.parse::<usize>() {
            writeln!(output, "{}", index_name[i-1]).unwrap()
        } else {
            writeln!(output, "{}", name_index[&input]).unwrap()
        }
    }
}