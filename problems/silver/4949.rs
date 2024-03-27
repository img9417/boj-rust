// https://www.acmicpc.net/problem/4949

use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap()
}

fn main() {
    let mut output = BufWriter::new(stdout().lock());

    loop {
        let mut stack: Vec<char> = vec![];
        let mut is_balance = true;
        let s = r();
        match s.as_str() {
            "." => break,
            _ => {
                for ch in s.chars() {
                    match ch {
                        '[' | '(' => stack.push(ch),
                        ']' | ')' => {
                            if stack.len() == 0 {
                                is_balance = false;
                                break;
                            }

                            let top = stack.last().unwrap();

                            if ch == ')' && *top == '(' {
                                stack.pop();
                            } else if ch == ']' && *top == '[' {
                                stack.pop();
                            } else {
                                is_balance = false;
                                break;
                            }
                        },
                        _ => continue
                    };

                }
            }
        }

        if stack.len() == 0 && is_balance {
            writeln!(output, "yes");
        } else {
            writeln!(output, "no");
        }
    }
}
