use std::io;

fn line_at_n(n: u32) -> String {
    let mut string_prototype = String::with_capacity(n as usize);

    //Loops through array, and adds together a bunch of *'s
    for _ in 0..n {
        string_prototype.push_str("*");
    }

    //Returns the prototype string
    string_prototype

}

fn main() {
    println!("Hello, world!");
    println!("How many rows would you like to have?");
    let mut n = String::new();    
    io::stdin().read_line(&mut n);

    let n: u32 = match n.trim().parse() {
        Ok(number) => number,
        Err(err) => panic!("{}", err)
    };

    for i in 0..n {
        println!("{}: {}", i, line_at_n(i+1))
    }
}