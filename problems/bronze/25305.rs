// https://www.acmicpc.net/problem/25305

use std::io::{stdin, stdout, BufRead, Write, BufWriter};

fn r() -> Vec<i32> {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().split_whitespace().flat_map(str::parse).collect()
}

fn display(arr: &mut [i32]) {
    for x in arr {
        print!("{} ", x);
    }

    println!();
}

fn main() {
    let nk = r();
    let mut numbers: Vec<i32> = r();

    // insertion sort
    for i in 1..nk[0] {
        display(&mut numbers);
        let key = numbers[i as usize];
        let mut j = i - 1;
        
        while j >= 0 && numbers[j as usize] > key {
            numbers[(j + 1) as usize] = numbers[j as usize];
            j -= 1;
        }
        numbers[(j+1) as usize] = key;
    }
    
    display(&mut numbers);
    let index = nk[0] - nk[1];

    let mut output = BufWriter::new(stdout().lock());
    writeln!(output, "{}", numbers[index as usize]).unwrap();
}