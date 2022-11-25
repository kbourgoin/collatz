#![feature(test)]

use clap::Parser;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod collatz;

// TODO: Fix 0 as a magic number to run forever

/// collatz -- run the 3x+1 problem
#[derive(Parser)]
struct Args {
    /// Start running at N
    #[clap(short, long, default_value_t = 1)]
    start: usize,
    /// Run for i numbers (0 runs forever)
    #[clap(short, long, default_value_t = 0)]
    count: usize,
    /// How many threads to use
    #[clap(short, long, default_value_t = 1)]
    threads: usize,
}

/// Entry point for output receiver
fn receiver(rx: Receiver<(usize, usize)>) {
    loop {
        if let Result::Ok((num, result)) = rx.recv() {
            println!("{:e}: {}", num, result);
        } else {
            return;
        }
    }
}

/// Run 3x+1 on start..end and print the results
fn run(start: usize, end: usize, threads: usize) {
    let (tx, rx): (Sender<(usize, usize)>, Receiver<(usize, usize)>) = mpsc::channel();
    let receiver_thread = thread::spawn(move || {
        receiver(rx);
    });
    collatz::solve(collatz::shortcut, start, end, tx, threads);
    receiver_thread.join().unwrap();
}

fn main() {
    let args = Args::parse();

    // Print message about what's about to go down
    let count_msg: String;
    if args.count == 0 {
        count_msg = "all".to_string();
    } else {
        count_msg = format!("{}", args.count);
    }
    let threads_msg: String;
    if args.threads == 1 {
        threads_msg = "single-threaded".to_string();
    } else {
        threads_msg = format!("using {} threads", args.threads);
    }
    println!(
        "Running for {} numbers starting at {}, {}",
        count_msg, args.start, threads_msg,
    );

    // Run the thing
    let end = match args.count {
        0 => 0,
        _ => args.start + args.count,
    };
    run(args.start, end, args.threads);
}
