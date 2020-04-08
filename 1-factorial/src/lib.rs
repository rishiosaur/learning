use std::io;

fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }

    n * factorial(n - 1)
}

fn main () {
    println!("Pick a number!");

    let mut number = String::new();
    io::stdin()
    .read_line(&mut number)
    .expect("There was an error while reading input.");

    let number = number.trim().parse().expect("Please enter a number.");

    println!("{}", factorial(number))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
