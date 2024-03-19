// https://www.acmicpc.net/problem/1620

use std::io::{self, Write, BufWriter};
use std::collections::HashMap;

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main() {
    let nm: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();
    let mut name_index: HashMap<String, i32> = HashMap::new();
    let mut index_name: HashMap<i32, String> = HashMap::new();

    for i in 1..=nm[0] {
        let pokemon = r();
        name_index.insert(pokemon.clone(), i);
        index_name.insert(i, pokemon);
    }

    let mut output = BufWriter::new(io::stdout().lock());
    for _ in 0..nm[1] {
        let input = r();

        match name_index.get(&input) {
            Some(index) => writeln!(output, "{index}").unwrap(),
            None => {
                let index: i32 = input.trim().parse().unwrap();
                writeln!(output, "{}", index_name.get(&index).unwrap()).unwrap();
            }
        }
    }
}