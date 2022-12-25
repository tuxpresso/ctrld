use std::fs::read_to_string;
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::{Duration, Instant};

use clap::Parser;
use pid::{Pid, PidBuilder, PidGains, PidLimits};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Period in millis
    #[arg(short)]
    period_ms: u32,

    /// Path to iio sysfs
    #[arg(short)]
    iio_path: String,

    /// Address of pwm service
    #[arg(short)]
    addr: String,

    /// Set point
    #[arg(long)]
    sp: f32,

    /// Maximum temperature
    #[arg(long)]
    max: i32,

    /// Proportional gain
    #[arg(long)]
    kp: f32,

    /// Integral gain
    #[arg(long)]
    ki: f32,

    /// Derivative gain
    #[arg(long)]
    kd: f32,
}

fn main() {
    let args = Args::parse();

    let sock = UdpSocket::bind("0.0.0.0:0").expect("failed to bind socket");
    sock.set_nonblocking(true)
        .expect("failed to set socket to nonblocking");

    let gains = PidGains::new(args.kp, args.ki, args.kd);
    let limit = args.period_ms as f32;
    let limits = PidLimits::new(limit, limit, limit, limit);
    let mut pid = Pid::from(PidBuilder::new(gains, limits, args.sp));

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

        match sock.send_to(&pulse_ms.to_ne_bytes(), &args.addr) {
            _ => (),
        };

        let elapsed = Instant::now() - start;
        sleep(period - elapsed);
    }
}
