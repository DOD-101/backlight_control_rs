use backlight_control_rs::*;
use clap::{Command, Parser, ValueHint};

fn main() {
    let args = Args::parse();

    if args.stats {
        println!("Max: {}", get_max_brightness());
        println!("Current: {}", get_brightness());
        return;
    }

    set_brightness(300).expect("ok");

    println!("Max brightness written to file.");
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "backlight_control_rs | a simple util for controlling the backlight brightness on your device"
)]
struct Args {
    /// The value to set / adjust the brightness by
    ///
    /// Example:
    ///
    /// +50
    ///
    /// -10
    ///
    /// 200
    ///
    // TODO: Add % support
    #[arg(value_hint = ValueHint::Other)]
    value: Option<String>,
    /// Print backlight information
    ///
    /// If this is used no value will be set, even if provided
    #[arg(short, long)]
    stats: bool,
}
