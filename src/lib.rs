use std::cmp::min;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

const BRIGHTNESS_BASE_PATH: &str = "/sys/class/backlight";

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

pub fn get_max_brightness() -> u16 {
    let max_brightness_path = vendor_path().join("max_brightness");

    fs::read_to_string(&max_brightness_path)
        .expect("Failed to read max_brightness file")
        .trim()
        .parse()
        .expect("Failed to parse max brightness. This should never happen.")
}

pub fn get_brightness() -> u16 {
    let brightness_path = vendor_path().join("brightness");

    fs::read_to_string(&brightness_path)
        .expect("Failed to read brightness file")
        .trim()
        .parse()
        .expect("Failed to parse brightness. This should never happen.")
}

pub fn set_brightness(value: u16) -> Result<(), std::io::Error> {
    let brightness_path = vendor_path().join("brightness");

    fs::write(
        brightness_path,
        min(value, get_max_brightness()).to_string(),
    )?;

    Ok(())
}