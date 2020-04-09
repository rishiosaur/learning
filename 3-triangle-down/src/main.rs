use std::io;

fn line_at_n(n:u32) -> String {
    let mut p_string = String::with_capacity(n as usize);

    for _ in 0..n {
        p_string.push_str("*");
    };

    p_string
}

fn main() {
    println!("How many lines would you like?" );
    let mut n = String::new();
    io::stdin().read_line(&mut n);

    let n = match n.trim().parse() {
        Ok(number) => number,
        Err(e) => panic!(e)
    };

    for i in 0..n {
        println!("{}: {}", i, line_at_n(n-i))
    };

}
