//! This lib allows for simple adjustment of the users backlight.
use std::{
    cmp::{max, min},
    fs, io,
    path::PathBuf,
    sync::OnceLock,
};

const BRIGHTNESS_BASE_PATH: &str = "/sys/class/backlight";

/// Internal function to get the path of the backlight
///
/// This is different from system to system, since it depends on the GPU.
fn vendor_path() -> &'static PathBuf {
    static VENDOR_DIR: OnceLock<PathBuf> = OnceLock::new();
    VENDOR_DIR.get_or_init(|| {
        let vendor_dir = fs::read_dir(BRIGHTNESS_BASE_PATH)
            .expect("Failed to read backlight dir.")
            .filter_map(Result::ok) // Filter out any errors
            .next() // Get the first entry
            .expect("Failed to get first Entry.") // Handle case where there are no entries
            .file_name(); // Get the file name directly

        PathBuf::from(BRIGHTNESS_BASE_PATH).join(vendor_dir)
    })
}

/// Gets the max brightness
///
/// ## Panics
///
/// Will only panic if it cannot parse the value in in the `max_brightness` file. This should never
/// happen. If it does this is an OS error.
pub fn get_max_brightness() -> io::Result<u16> {
    let max_brightness_path = vendor_path().join("max_brightness");

    Ok(fs::read_to_string(&max_brightness_path)?
        .trim()
        .parse()
        .expect("Failed to parse max brightness. This should never happen."))
}

/// Gets the current brightness
///
/// ## Panics
///
/// Will only panic if it cannot parse the value in in the `brightness` file. This should never
/// happen. If it does this is an OS error.
pub fn get_brightness() -> io::Result<u16> {
    let brightness_path = vendor_path().join("brightness");

    Ok(fs::read_to_string(&brightness_path)?
        .trim()
        .parse()
        .expect("Failed to parse brightness. This should never happen."))
}

/// Attempts to set the brightness
///
/// This will fail if you lack the proper permissions.
pub fn set_brightness(value: u16) -> Result<(), std::io::Error> {
    let brightness_path = vendor_path().join("brightness");

    fs::write(
        brightness_path,
        min(value, get_max_brightness()?).to_string(),
    )?;

    Ok(())
}

/// Convenience wrapper around set_brightness for adjusting relative to the current value
///
/// # Parameters
///
/// - `value`: The value to adjust by.
///
/// - `percentage`: Will treat the value as a percentage to adjust by, rather than an absolute value.
pub fn adjust_brightness_relative(value: i16, percentage: bool) -> io::Result<()> {
    let brightness: i16 = get_brightness()?.try_into().unwrap();

    if percentage {
        let max_brightness: i16 = get_max_brightness()?.try_into().unwrap();

        let new_brightness: u16 =
            (max_brightness as f32 * (value as f32 / 100.0) + brightness as f32).max(0.0) as u16;

        set_brightness(new_brightness)?;

        return Ok(());
    }

    let new_brightness: i16 = brightness + value;

    set_brightness(max(0, new_brightness).try_into().unwrap())
}

/// Convenience wrapper around set_brightness for setting absolute values
///
/// ## Parameters
///
/// - `value`: The value to set to.
///
/// - `percentage`: Will treat the value as a percentage of the max to set to, rather than an absolute value.
pub fn adjust_brightness_absolute(value: u16, percentage: bool) -> io::Result<()> {
    if percentage {
        let max_brightness: u16 = get_max_brightness()?;

        let new_brightness: u16 = (max_brightness as f32 * (value as f32 / 100.0)).max(0.0) as u16;

        set_brightness(new_brightness)?;

        return Ok(());
    }

    set_brightness(value)
}
