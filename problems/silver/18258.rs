use std::io::{stdin, stdout, BufRead, BufWriter, Write};
use std::collections::VecDeque;

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().to_owned()
}

fn main() {
    let t = r().parse::<u32>().unwrap();
    let mut q: VecDeque<u32> = Default::default();

    let mut output = BufWriter::new(stdout().lock());
    for _ in 0..t {
        let command = r();

        if command.starts_with("push") {
            let x = command.split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
            q.push_back(x);
            continue;
        }

        match command.as_str() {
            "front" => {
                if q.is_empty() {
                    writeln!(output, "-1");
                } else {
                    writeln!(output, "{}", q.front().unwrap());
                }
            },
            "back" => {
                if q.is_empty() {
                    writeln!(output, "-1");
                } else {
                    writeln!(output, "{}", q.back().unwrap());
                }
            },
            "empty" => {
                writeln!(output, "{}", q.is_empty() as u32);
            },
            "size" => {
                writeln!(output, "{}", q.len());
            },
            "pop" => {
                if q.is_empty() {
                    writeln!(output, "-1");
                } else {
                    let f = q.pop_front().unwrap();
                    writeln!(output, "{f}");
                }
            },
            _ => panic!()
        }
    }
}