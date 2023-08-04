pub mod escape_sequences {
    use std::{fmt, io};

    pub const WHITE_ON_DEFAULT: &str = "\x1b[97;49m";
    pub const BLACK_FOREGROUND: &str = "\x1b[30m";
    pub const DARK_GRAY_FOREGROUND: &str = "\x1b[38;5;8m";
    pub const GREEN_FOREGROUND: &str = "\x1b[32m";
    pub const YELLOW_FOREGROUND: &str = "\x1b[93m";
    pub const RED_FOREGROUND: &str = "\x1b[31m";

    pub const RESET: &str = "\x1b[0m";

    pub fn write_grayscale(f: &mut fmt::Formatter<'_>, brightness: u8) -> fmt::Result {
        write!(f, "{}\x1b[48;5;{}m", BLACK_FOREGROUND, brightness)
    }

    pub fn cursor_up(f: &mut impl io::Write, n: usize) -> io::Result<()> {
        write!(f, "\x1b[{}F", n)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
