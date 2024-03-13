use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut input: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();

    /* METHOD 1 */
    let input: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();
    let (a, b, c) = (input[0], input[1], input[2]);

    match (a,b,c) {
        (a,b,c) if a>=b&&a<=c => println!("{a}"),
        (a,b,c) if a>=c&&a<=b => println!("{a}"),
        (a,b,c) if b>=a&&b<=c => println!("{b}"),
        (a,b,c) if b>=c&&b<=a => println!("{b}"),
        _ => println!("{c}"),
    }

    /* METHOD 2 */
    // input.sort();
    // println!("{}", input[1]);
}