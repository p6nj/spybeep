#![allow(unused_parens)]
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(super) struct Args {
    /// Volume of beeps (u8: 0-255)
    #[arg(short, long, default_value_t = 127)]
    pub(super) volume: u8,
    /// Duration of each beep in milliseconds
    #[arg(short, long, default_value_t = 100)]
    pub(super) duration: u64,
    /// Comma-separated list of keys to use
    #[arg(
        long,
        default_value_t = ("A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,1,2,3,4,5,6,7,8,9,0,LeftAlt,RightAlt,LeftControl,RightControl,BackSpace,Tab,Enter,Escape,Space,PageUp,PageDown,Home,Left,Up,Right,Down,Print,PrintScreen,Insert,Delete,LeftWindows,RightWindows,Comma,Period,Slash,SemiColon,Apostrophe,LeftBrace,BackwardSlash,RightBrace,Grave,F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24,NumLock,ScrollLock,CapsLock,Numpad0,Numpad1,Numpad2,Numpad3,Numpad4,Numpad5,Numpad6,Numpad7,Numpad8,Numpad9,Multiply,Add,Separator,Subtract,Decimal,Divide".to_string())
    )]
    pub(super) keys: String,
    #[command(subcommand)]
    pub(super) notes: Option<Mode>,
}

#[derive(Subcommand, Debug)]
pub(super) enum Mode {
    TET {
        /// Number of notes in the scale (tempered)
        #[arg(short, long, default_value_t = 12)]
        notes: u8,
        /// First note (shifted from A0)
        #[arg(short, long, default_value_t = 12*2)]
        first: u8,
    },
    Scale {
        /// Key of the scale (shifted from A0)
        #[arg(short, long, default_value_t = 12*2)]
        key: u8,
        /// First note (shifted from the key)
        #[arg(short, long, default_value_t = 0)]
        first: u8,
        /// Medieval mode (shifted from C)
        #[arg(short, long, default_value_t = 0)]
        mode: u8,
    },
}

impl Default for Mode {
    fn default() -> Self {
        Self::TET {
            notes: 24,
            first: 24 * 3,
        }
    }
}
