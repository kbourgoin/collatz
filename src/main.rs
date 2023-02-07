#![feature(test)]

use clap::Parser;
use std::cmp::max;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use thousands::Separable;

pub mod collatz;

/// collatz -- run the 3x+1 problem on some numbers or something
#[derive(Parser)]
struct Args {
    /// Start running at N
    #[clap(short, long, default_value_t = 1)]
    start: usize,
    /// Run for i numbers (0 runs forever)
    #[clap(short, long, default_value_t = 0)]
    count: usize,
    /// How big a batch to send to each thread
    #[clap(short, long, default_value_t = 10_000_000)]
    batch_size: usize,
    /// How many threads to use
    #[clap(short, long, default_value_t = num_cpus::get())]
    threads: usize,
}

/// Print progressive solve times and status
fn receiver(rx: Receiver<collatz::BatchSummary>) {
    let mut solves = 0;
    let mut dur = Duration::new(0, 0);
    let mut max_steps = 0;
    let mut msg_num = 0;
    println!("Total Solves\tOverall solves/s\tBatch Duration\tBatch solves/s\tMax steps to solve");
    loop {
        if let Result::Ok(summary) = rx.recv() {
            // Only print every 20th message or it's too noisy.
            msg_num += 1;
            if msg_num % 20 != 0 {
                continue;
            }

            /// Print out some stats about the batch
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
                "{:.2e}\t\t{:.3e}\t\t\t{:.3}ms\t\t{:.2e}\t\t{}",
                solves,
                rate,
                batch_dur.as_secs_f32() * 1000.0,
                batch_rate,
                max_steps,
            );
        } else {
            // Done processing. Print a final summary and exit.
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
fn run(start: usize, end: usize, batch_size: usize, threads: usize) {
    let (tx, rx): (
        Sender<collatz::BatchSummary>,
        Receiver<collatz::BatchSummary>,
    ) = mpsc::channel();
    let receiver_thread = thread::spawn(move || {
        receiver(rx);
    });
    collatz::solve(start, end, tx, batch_size, threads);
    receiver_thread.join().unwrap();
}

fn main() {
    let args = Args::parse();

    // Print message about what's about to go down
    let end_msg: String;
    if args.count == 0 {
        end_msg = "∞".to_string();
    } else {
        end_msg = format!("{}", (args.start + args.count).separate_with_commas());
    }

    println!(
        "Run starting:\n  \
        - start: {}\n  \
        - end: {}\n  \
        - batch size: {}\n  \
        - threads: {}\n",
        args.start.separate_with_commas(),
        end_msg,
        args.batch_size.separate_with_commas(),
        args.threads,
    );

    let end = match args.count {
        0 => 0,
        _ => args.start + args.count,
    };

    // Run the thing
    if end == 0 {
        // Run forever by solving 1,000 batches at a time
        let mut start = args.start;
        let mut end = start + args.batch_size * 1_000;
        loop {
            print!(
                "Starting infini-batch [{:.2e}, {:.2e}]\n-----\n\n",
                start, end
            );
            run(start, end, args.batch_size, args.threads);
            start = end;
            end = start + args.batch_size * 1_000;
        }
    } else {
        run(args.start, end, args.batch_size, args.threads);
    }
}
