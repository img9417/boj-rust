use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut n: u32 = n.trim().parse().unwrap();   
    let mut i = 666;

    loop {
        if n == 0 { 
            break;
        }
        match i.to_string().find("666") {
            Some(_) => {n -= 1; i+=1},
            None => i += 1
        }
    }

    println!("{}", i-1);
}