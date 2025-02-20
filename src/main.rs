use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

use clap::Parser;
use pid::Pid;

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

        let pulse_ms: u32 = match line.trim().parse::<i32>() {
            Ok(temp) if (0..=args.max).contains(&temp) => {
                pid.next_control_output(temp as f32).output as u32
            }
            _ => 0,
        };
        println!("{}", pulse_ms);

        let elapsed = Instant::now() - start;
        sleep(period - elapsed);
    }
}
