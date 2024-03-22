https://www.acmicpc.net/problem/1269

use std::io::{stdin, stdout, BufRead, Write, BufWriter};
use std::collections::HashSet;

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().to_owned()

}
fn main() {
    r();

    let set_a: HashSet<i64> = r().split_whitespace().flat_map(str::parse).collect();
    let set_b: HashSet<i64> = r().split_whitespace().flat_map(str::parse).collect();

    let mut ab_cnt = 0;
    let mut ba_cnt = 0;

    for v in set_a.iter() {
        if set_b.contains(&v) == false {
            ab_cnt += 1;
        }
    }

    for v in set_b.iter() {
        if set_a.contains(&v) == false {
            ba_cnt += 1;
        }
    }

    let mut output = BufWriter::new(stdout().lock());
    writeln!(output, "{}", ab_cnt + ba_cnt).unwrap();
}