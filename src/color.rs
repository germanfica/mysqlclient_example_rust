// color.rs

/// Returns the ANSI escape code for bright red text.
///
/// # Examples
///
/// ```
/// println!("{}Error occurred!{}", error_color(), normal_color());
/// ```
///
/// This will print the text "Error occurred!" in bright red.
pub fn error_color() -> &'static str {
    "\x1b[1;31m" // Bright red
}

/// Returns the ANSI escape code for bright green text.
///
/// # Examples
///
/// ```
/// println!("{}Operation successful!{}", success_color(), normal_color());
/// ```
///
/// This will print the text "Operation successful!" in bright green.
pub fn success_color() -> &'static str {
    "\x1b[1;32m" // Bright green
}

/// Returns the ANSI escape code for bright yellow text.
///
/// # Examples
///
/// ```
/// println!("{}Warning: Check your input!{}", warning_color(), normal_color());
/// ```
///
/// This will print the text "Warning: Check your input!" in bright yellow.
pub fn warning_color() -> &'static str {
    "\x1b[1;33m" // Bright yellow
}

/// Returns the ANSI escape code for white text on a bright red background.
///
/// # Examples
///
/// ```
/// println!("{}Critical setup error!{}", error_setup_color(), normal_color());
/// ```
///
/// This will print the text "Critical setup error!" with a bright red background and white text.
pub fn error_setup_color() -> &'static str {
    "\x1b[101;37m" // White text on bright red background
}

/// Returns the ANSI escape code to reset text formatting to default.
///
/// # Examples
///
/// ```
/// println!("{}This is normal text", normal_color());
/// ```
///
/// This will reset the text formatting to the default terminal color.
pub fn normal_color() -> &'static str {
    "\x1b[0m" // Reset to default color
}
