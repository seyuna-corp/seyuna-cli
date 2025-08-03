use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use spinach::{RunningSpinner, Spinner};

/// Generates a gradient-colored string from `start` to `end` RGB values.
///
/// # Arguments
/// * `text` - The text to color.
/// * `start` - The starting RGB color tuple.
/// * `end` - The ending RGB color tuple.
///
/// # Returns
/// `String` - With each character colored along the gradient.
pub fn gradient(text: &str, start: (u8, u8, u8), end: (u8, u8, u8)) -> String {
    let len = text.chars().count().max(1) as f32;
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            let t = i as f32 / len;
            let r = (start.0 as f32 * (1.0 - t) + end.0 as f32 * t) as u8;
            let g = (start.1 as f32 * (1.0 - t) + end.1 as f32 * t) as u8;
            let b = (start.2 as f32 * (1.0 - t) + end.2 as f32 * t) as u8;
            format!("{}", c.truecolor(r, g, b))
        })
        .collect()
}

/// Returns the primary gradient style for CLI text.
///
/// # Arguments
/// * `text` - The text to style.
///
/// # Returns
/// `String` - With the primary gradient applied.
pub fn primary_text(text: &str) -> String {
    gradient(text, (0, 255, 135), (96, 239, 255))
}

/// Returns the secondary gradient style for CLI text.
///
/// # Arguments
/// * `text` - The text to style.
///
/// # Returns
/// `String` - With the secondary gradient applied.
pub fn secondary_text(text: &str) -> String {
    gradient(text, (255, 15, 123), (248, 155, 41))
}

/// Returns error style for CLI text.
///
/// # Arguments
/// * `text` - The text to style.
///
/// # Returns
/// `String` - With an error prefix.
pub fn error_text(text: &str) -> String {
    format!("{} {}", secondary_text("Error:").bold(), text)
}

/// A static checkmark symbol with primary gradient styling.
pub static CHECKMARK: Lazy<String> = Lazy::new(|| primary_text("✔"));

/// Starts a spinner with the given text and a short delay for effect.
///
/// # Arguments
/// * `text` - The spinner's initial message.
///
/// # Returns
/// `RunningSpinner` - Instance.
pub fn spinner_start(text: &str) -> RunningSpinner {
    let s = Spinner::new(text).start();
    s
}

/// Updates the spinner's message and waits briefly.
///
/// # Arguments
/// * `s` - The running spinner.
/// * `text` - The new message to display.
pub fn spinner_progress(s: &RunningSpinner, text: &str) {
    s.text(text).update();
}

/// Stops the spinner, sets a checkmark, and displays a final message.
///
/// # Arguments
/// * `s` - The running spinner.
/// * `text` - The completion message.
pub fn spinner_done(s: &RunningSpinner, text: &str) {
    s.text(&primary_text(text)).symbol(&CHECKMARK).stop();
}
