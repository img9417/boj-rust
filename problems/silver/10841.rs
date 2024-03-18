// https://www.acmicpc.net/problem/10814

use std::io::{stdin, stdout, BufWriter, BufRead, Write};

fn r() -> String {
    let input = stdin().lock();
    let mut lines = input.lines();
    lines.next().unwrap().unwrap().to_owned()
}

fn main() {
    let t: i32 = r().trim().parse().unwrap();
    let mut users: Vec<(i32, String)> = vec![];

    for _ in 0..t {
        let input: Vec<String> = r().split_whitespace().flat_map(str::parse).collect();
        let age: i32 = input[0].trim().parse().unwrap();
        let name: String = input[1].to_owned();

        users.push((age, name));
    }

    users.sort_by(|a,b| (a.0).cmp(&b.0));

    let mut output = BufWriter::new(stdout().lock());
    for user in users {
        writeln!(output, "{} {}", user.0, user.1).unwrap();
    }
}