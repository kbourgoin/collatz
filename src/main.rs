#![feature(test)]

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
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
    /// Where to end (0 runs forever)
    #[clap(short, long, default_value_t = 0)]
    end: usize,
    /// Override default batch size of `num_to_solve / (threads*2) `
    #[clap(short, long)]
    batch_override: Option<usize>,
    /// How many threads to use
    #[clap(short, long, default_value_t = num_cpus::get())]
    threads: usize,
}

impl Args {
    /// Batch size is computed after args are validated because it may be overridden.
    // It's a bit ugly here, but expedient because Clap derive uses everything in the struct
    pub fn batch_size(&self) -> usize {
        match self.batch_override {
            Some(size) => size,
            None => {
                match self.end {
                    // For infinite runs, use a pretty-good default
                    0 => 200_000_000,
                    _ => (self.end - self.start) / (self.threads * 2),
                }
            }
        }
    }
}

/// Print progressive solve times and status
fn receiver(rx: Receiver<collatz::BatchSummary>) {
    let mut solves = 0;
    let mut dur = Duration::new(0, 0);
    let mut max_steps = 0;
    println!("Total Solves\tOverall solves/s\tBatch Duration\tBatch solves/s\tMax steps to solve");
    loop {
        if let Result::Ok(summary) = rx.recv() {
            // Print out some stats about the batch
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

/// Parse/validate arguments and handle any that are computed at runtime
fn get_args() -> Args {
    let args = Args::parse();

    // Ensure end == 0 || end > start
    if args.end > 0 && args.end < args.start {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ArgumentConflict,
            "`end` must be 0 or greater than start",
        )
        .exit();
    }
    args
}

fn main() {
    let args = get_args();

    // Print message about what's about to go down
    println!(
        "Running with settings:\n  \
        - start: {}\n  \
        - end: {}\n  \
        - batch size: {}\n  \
        - threads: {}\n",
        args.start.separate_with_commas(),
        match args.end {
            0 => String::from("∞"),
            _ => args.end.separate_with_commas(),
        },
        args.batch_size().separate_with_commas(),
        args.threads,
    );

    // Run the thing
    if args.end == 0 {
        // Run forever by calling `run` repeatedly`
        let step_size = 20_000_000_000;
        let mut start = args.start;
        let mut end = start + step_size;
        loop {
            print!(
                "Starting infini-batch [{:.2e}, {:.2e}]\n-----\n\n",
                start, end
            );
            run(start, end, args.batch_size(), args.threads);
            start = end;
            end = start + step_size;
        }
    } else {
        run(args.start, args.end, args.batch_size(), args.threads);
    }
}
