use clap::Parser;
use cli::Args;
use freqiterator::{FreqGenerator, ScaleGenerator, A0};
use mki::Keyboard::{self, *};
use rodio::{source::SineWave, OutputStream, Source};
use std::{thread::sleep, time::Duration};
mod cli;

fn main() {
    let args = Args::parse();
    let (_stream, handle) = OutputStream::try_default().expect("can't get any sound device");
    {
        let mut keys = [
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z,
            Number0,
            Number1,
            Number2,
            Number3,
            Number4,
            Number5,
            Number6,
            Number7,
            Number8,
            Number9,
            LeftAlt,
            RightAlt,
            LeftControl,
            RightControl,
            BackSpace,
            Tab,
            Enter,
            Escape,
            Space,
            PageUp,
            PageDown,
            Home,
            Left,
            Up,
            Right,
            Down,
            Print,
            PrintScreen,
            Insert,
            Delete,
            LeftWindows,
            RightWindows,
            Comma,         // ,<
            Period,        // .>
            Slash,         // /?
            SemiColon,     // ;:
            Apostrophe,    // '"
            LeftBrace,     // [{
            BackwardSlash, // \|
            RightBrace,    // ]}
            Grave,         // `~
            F1,
            F2,
            F3,
            F4,
            F5,
            F6,
            F7,
            F8,
            F9,
            F10,
            F11,
            F12,
            F13,
            F14,
            F15,
            F16,
            F17,
            F18,
            F19,
            F20,
            F21,
            F22,
            F23,
            F24,
            NumLock,
            ScrollLock,
            CapsLock,
            Numpad0,
            Numpad1,
            Numpad2,
            Numpad3,
            Numpad4,
            Numpad5,
            Numpad6,
            Numpad7,
            Numpad8,
            Numpad9,
            Multiply,
            Add,
            Separator,
            Subtract,
            Decimal,
            Divide,
        ];
        keys.sort();
        keys
    }
    .iter()
    .zip(ScaleGenerator::new(
        FreqGenerator::new(A0, args.scale as f32).skip(args.key.into()),
        0,
    ))
    .zip(ScaleGenerator::new(
        FreqGenerator::new(A0, args.scale as f32).skip(args.key.into()),
        5,
    )) // todo: zip both major and minor scales
    .for_each(|((key, maj), min)| {
        let handle = handle.clone();
        key.bind(move |k| {
            println!("{k}");
            handle
                .play_raw(
                    SineWave::new(
                        if Keyboard::LeftShift.is_pressed() || Keyboard::RightShift.is_pressed() {
                            maj
                        } else {
                            min
                        },
                    )
                    .take_duration(Duration::from_millis(args.duration))
                    .fade_in(Duration::from_millis(10))
                    .amplify(args.volume as f32 / u8::MAX as f32),
                )
                .unwrap()
        })
    });
    sleep(Duration::from_secs(u64::MAX));
}
