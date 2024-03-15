// https://www.acmicpc.net/problem/1018

use std::io;

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn check_min(color: char, sy: usize, sx: usize, mut min: i32, chessboard: &Vec<Vec<char>>) -> i32 {
    let mut cnt = 0;

    for y in sy..sy+8 {
        for x in sx..sx+8 {
            if (y+x)&1 != 1 {
                if chessboard[y as usize][x as usize] != color {
                    cnt += 1;
                }
            } else {
                if chessboard[y as usize][x as usize] == color {
                    cnt += 1;
                }
            }
        }
    }

    if cnt < min {
        min = cnt;
    }

    min

}

fn main() {
    let xy: Vec<usize> = r().split_whitespace().flat_map(str::parse).collect();
    let mut min = 2500;
    let mut chessboard: Vec<Vec<char>> = vec![];

    for _ in 0..xy[0] {
        chessboard.push(r().chars().collect());
    }

    for y in 0..xy[0]-7 {
        for x in 0..xy[1]-7 {
            min = check_min('B', y, x, min, &chessboard);
            min = check_min('W', y, x, min, &chessboard);
        }
    }

    println!("{min}");
}