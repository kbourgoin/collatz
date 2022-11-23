#![feature(test)]
mod collatz;

fn main() {
    // println!("Found in: {}", collatz::recursive(27));
    for n in 1..10 {
        let result = collatz::shortcut(n);
        println!("{}: {}", n, result);
    }
}
