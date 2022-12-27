use std::fs::read_to_string;
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::{Duration, Instant};

use ciborium::ser::into_writer;
use clap::Parser;
use pid::Pid;
use serde::Serialize;

mod args;
use crate::args::Args;

#[derive(Serialize)]
struct PwmMessage {
    pulse_ms: u32,
}

fn main() {
    let args = Args::parse();

    let sock = UdpSocket::bind("0.0.0.0:0").expect("failed to bind socket");
    sock.set_nonblocking(true)
        .expect("failed to set socket to nonblocking");

    let limit = args.period_ms as f32;
    let mut pid = Pid::new(args.sp, limit);
    pid.p(args.kp, limit);
    pid.i(args.ki, limit);
    pid.d(args.kd, limit);

    let period = Duration::from_millis(args.period_ms.into());
    loop {
        let start = Instant::now();

        let pulse_ms: u32 = match read_to_string(&args.iio_path) {
            Ok(string) => match string.trim().parse::<i32>() {
                Ok(temp) if (0..=args.max).contains(&temp) => {
                    pid.next_control_output(temp as f32).output as u32
                }
                _ => 0,
            },
            _ => 0,
        };
        println!("{}", pulse_ms);

        let mut buf: [u8; 1024] = [0; 1024];
        let msg = PwmMessage { pulse_ms };

        match into_writer(&msg, &mut buf[0..1024]) {
            _ => (),
        }
        match sock.send_to(&buf, &args.addr) {
            _ => (),
        };

        let elapsed = Instant::now() - start;
        sleep(period - elapsed);
    }
}
