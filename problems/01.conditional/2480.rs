use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let dies: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();
    let a = dies[0];
    let b = dies[1];
    let c = dies[2];

    match (a,b,c) {
        (a,b,c) if a==b && b==c => println!("{}",10000+1000*a),
        (a,b,c) if a==b || a==c => println!("{}", 1000+100*a),
        (_,b,c) if b==c => println!("{}", 1000+100*b),
        (a,b,c) if a>b && a>c => println!("{}", 100*a),
        (a,b,c) if b>a && b>c => println!("{}", 100*b),
        _ => println!("{}", 100*c),
    }
}