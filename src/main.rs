use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

use clap::Parser;
use pid::{ControlOutput, Pid};

mod args;
use crate::args::Args;

fn main() {
    let args = Args::parse();

    let limit = args.period_ms as f32;
    let mut pid = Pid::new(args.sp, limit);
    pid.p(args.kp, limit);
    pid.i(args.ki, limit);
    pid.d(args.kd, limit);

    // Use channels to achieve nonblocking reads from stdin
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut lines = io::stdin().lock().lines();
        loop {
            let line = lines.next().unwrap().unwrap();
            tx.send(line).unwrap();
        }
    });

    let period = Duration::from_millis(args.period_ms.into());
    loop {
        let start = Instant::now();

        // Drain the channel and keep the newest input
        let mut line = String::new();
        while let Ok(data) = rx.try_recv() {
            line = data;
        }

        let pid_output = match line.trim().parse::<i32>() {
            Ok(temp) if (0..=args.max).contains(&temp) => {
                pid.next_control_output(temp as f32)
            }
            _ => ControlOutput {output: 0.0, p: 0.0, i: 0.0, d: 0.0},
        };
        println!("{}", pid_output.output as u32);
        if args.verbose {
            eprintln!("{} {} {}", pid_output.p, pid_output.i, pid_output.d);
        }

        let elapsed = Instant::now() - start;
        sleep(period - elapsed);
    }
}
