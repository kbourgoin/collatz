#![feature(test)]

use clap::Parser;
use std::cmp::{max};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration};
use thousands::Separable;

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

/// Print progressive solve times and status
fn receiver(rx: Receiver<collatz::BatchSummary>) {
    let mut solves = 0;
    let mut dur = Duration::new(0, 0);
    let mut max_steps = 0;
    println!("Total Solves\tOverall solves/s\tBatch Duration\tBatch solves/s\tMax steps to solve");
    loop {
        if let Result::Ok(summary) = rx.recv() {
            let batch_solves = summary.end - summary.start;
            let batch_dur = summary
                .end_time
                .duration_since(summary.start_time)
                .expect("invalid SystemTime");
            let batch_rate = batch_solves as f32 / batch_dur.as_secs_f32();

            dur += batch_dur;
            solves += batch_solves;
            max_steps = max(summary.max_steps, max_steps);
            let rate = solves as f32 / dur.as_secs_f32();

            println!(
                "{}\t\t{:.2e}\t\t\t{:?}\t\t{:.2e}\t\t{}",
                solves.separate_with_commas(),
                rate,
                batch_dur,
                batch_rate,
                max_steps,
            );
        } else {
            let rate = solves as f32 / dur.as_secs_f32();
            println!(
                "\n-----\nTotal Solves: {}\nProcessing Time: {:?}\nSolves/s: {}",
                solves.separate_with_commas(),
                dur,
                rate.separate_with_commas(),
            );
            return;
        }
    }
}

/// Run 3x+1 on start..end and print the results
fn run(start: usize, end: usize, threads: usize) {
    let (tx, rx): (
        Sender<collatz::BatchSummary>,
        Receiver<collatz::BatchSummary>,
    ) = mpsc::channel();
    let receiver_thread = thread::spawn(move || {
        receiver(rx);
    });
    collatz::solve(start, end, tx, threads);
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
