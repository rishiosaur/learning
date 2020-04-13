fn fib (n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1
    }

    else {
        // println!("Processing: {}", n);
        return fib(n-1) + fib(n-2)
    }
}

fn main() {
    let n = 32;
    println!("{}", fib(n) );
}

