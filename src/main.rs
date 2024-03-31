use clap::Parser;
use cli::{Args, Mode};
use freqiterator::{FreqGenerator, ScaleGenerator, A0};
use mki::Keyboard;
use rodio::{source::SineWave, OutputStream, Source};
use std::{str::FromStr, thread::sleep, time::Duration};
mod cli;

fn mode_redirect(mode: Mode) -> Box<dyn Iterator<Item = f32>> {
    match mode {
        Mode::Scale { key, first, mode } => Box::new(
            ScaleGenerator::new(FreqGenerator::new(A0, 12f32).skip(key.into()), mode.into())
                .skip(first.into()),
        ),
        Mode::TET { notes, first } => {
            Box::new(FreqGenerator::new(A0, notes.into()).skip(first.into()))
        }
    }
}

fn main() {
    let args = Args::parse();
    let (_stream, handle) = OutputStream::try_default().expect("can't get any sound device");
    args.keys
        .split(',')
        .map(|k| Keyboard::from_str(k).expect(format!("Unknown key: '{k}'").as_str()))
        .zip(mode_redirect(args.notes.unwrap_or_default()))
        .for_each(|(key, f)| {
            let handle = handle.clone();
            key.bind(move |k| {
                println!("{k}");
                handle
                    .play_raw(
                        SineWave::new(f)
                            .take_duration(Duration::from_millis(args.duration))
                            .fade_in(Duration::from_millis(10))
                            .amplify(args.volume as f32 / u8::MAX as f32),
                    )
                    .unwrap()
            })
        });
    sleep(Duration::from_secs(u64::MAX));
}
