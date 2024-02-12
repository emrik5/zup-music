use std::{error::Error, fs};

use midly::{Smf, Track};

enum Instruction {
    Note { note: u8, len: u32 },
    Pause(u8),
    ChangeWave(u8),
    Sweep { start: u8, end: u8, len: u32 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "midi/test1.mid";
    let file = fs::read(filepath)?;
    let smf = Smf::parse(&file)?;
    println!("{:#?}", smf);
    Ok(())
}
fn parse_track(track: &Track) {
    for &event in track {
        match event.kind {
            midly::TrackEventKind::Midi { channel, message } => todo!(),
            midly::TrackEventKind::Meta(_) => todo!(),
            _ => {}
        }
    }
}
