fn line_at_n(n: u32, total: u32) -> String {
    /*
     Every line is split up into 3 different parts: a space, a character space, and an ending space. The starting and ending space will always be ((t-i)/2).
     */

     let mut spaces = String::new();
     let mut chars = String::new();
     
     for _ in 0..((total-n)/2) {
         spaces.push_str(" ");
     };

     for _ in 0..(n) {
         chars.push_str("*");
     };

     return format!("{}{}{}", spaces, chars, spaces);
}

fn main() {
    const ROWS: u32 = 9;

    for i in 0..ROWS {
        if i % 2 == 1 {
            print!("{}", i);
            println!("{}", line_at_n(i, ROWS))
        }
    }

    
    //This function should only work for odd numbers.
}