use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Period in millis
    #[arg(short)]
    pub period_ms: u32,

    /// Path to iio sysfs
    #[arg(short)]
    pub iio_path: String,

    /// Address of pwm service
    #[arg(short)]
    pub addr: String,

    /// Set point
    #[arg(long)]
    pub sp: f32,

    /// Maximum temperature
    #[arg(long)]
    pub max: i32,

    /// Proportional gain
    #[arg(long)]
    pub kp: f32,

    /// Integral gain
    #[arg(long)]
    pub ki: f32,

    /// Derivative gain
    #[arg(long)]
    pub kd: f32,
}
