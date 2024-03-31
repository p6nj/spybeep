use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(super) struct Args {
    /// Volume of beeps (u8: 0-255)
    #[arg(short, long, default_value_t = 127)]
    pub(super) volume: u8,
    /// Duration of each beep in milliseconds
    #[arg(short, long, default_value_t = 100)]
    pub(super) duration: u64,
    /// Number of notes in the scale (tempered)
    #[arg(short, long, default_value_t = 12)]
    pub(super) scale: u8,
    /// Key of the scale or first note (shifted from A0)
    #[arg(short, long, default_value_t = 12*3)]
    pub(super) key: u8,
}
