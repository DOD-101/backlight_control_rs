use backlight_control_rs::*;
use clap::{Parser, ValueHint};
use regex::Regex;

fn main() {
    let args = Args::parse();

    if args.stats {
        println!("Max: {}", get_max_brightness());
        println!("Current: {}", get_brightness());
        return;
    }

    if let Some(val) = args.value {
        let percentage = val.ends_with("%");

        let end_pos = if percentage { 1 } else { 0 };

        let value_string = val.chars().take(val.len() - end_pos).collect::<String>();

        // NOTE: Unwrapping here *should* be safe since we know the string has len of at least 1
        if let '+' | '-' = val.chars().next().unwrap() {
            let value: i16 = value_string.parse().unwrap();

            adjust_brightness_relative(value, percentage)
                .expect("Failed to adjust brightness relatively");
        } else {
            let value: u16 = value_string.parse().unwrap();

            adjust_brightness_absolute(value, percentage)
                .expect("Failed to adjust brightness absolutely");
        }
    }

    // set_brightness(300).expect("ok");
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "backlight_control_rs | a simple util for controlling the backlight brightness on your device"
)]
struct Args {
    /// The value to set / adjust the brightness by
    ///
    /// Examples:
    ///
    /// +50
    ///
    /// -10
    ///
    /// 200
    ///
    /// 50%
    ///
    /// +10%
    #[arg(value_hint = ValueHint::Other, value_parser = value_validator, allow_hyphen_values = true)]
    value: Option<String>,
    /// Print backlight information
    ///
    /// If this is used no value will be set, even if provided
    #[arg(short, long)]
    stats: bool,
}

fn value_validator(value: &str) -> Result<String, String> {
    if value.is_empty() {
        return Ok("".to_string());
    }

    let re = Regex::new(r"^[+-]?[0-9]+%?$").unwrap();

    if re.is_match(value) {
        Ok(value.to_string())
    } else {
        Err("Value provided does not match proper form.".to_string())
    }
}
