#![feature(test)]

use clap::Parser;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

mod collatz;

/// collatz -- run the 3x+1 problem
#[derive(Parser)]
struct Args {
    /// Start running at N
    #[clap(short, long, default_value_t = 1)]
    start: usize,
    /// Run for i numbers (0 runs forever)
    #[clap(short, long, default_value_t = 0)]
    count: usize,
}

/// Entry point for output receiver
fn receiver(rx: Receiver<(usize, usize)>) {
    while true {
        let (num, result) = rx.recv().unwrap();
        println!("{}: {}", num, result);
    }
}

fn main() {
    let (tx, rx): (Sender<(usize, usize)>, Receiver<(usize, usize)>) = mpsc::channel();

    // Start receiver thread

    // Run solver
    collatz::solve_mt(collatz::shortcut, 1, 10, tx);

    /*
    let args = Args::parse();
    let count_msg: String;
    if args.count == 0 {
        count_msg = "all".to_string();
    } else {
        count_msg = format!("{}", args.count);
    }
    println!(
        "Running for {} numbers starting at {}",
        count_msg, args.start
    );

    let mut i = 0;
    while args.count == 0 || i < args.count {
        let result = collatz::shortcut(args.start + i);
        // Print every million so we saturate CPU
        if (i + 1) % 1_000_000 == 0 {
            println!("{:e}: {}", args.start + i, result);
        }
        i += 1;
    }
    */
}
