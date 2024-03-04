use std::{error::Error, fs};

use export::export_to_zup;
use midly::{Smf, Track};

pub mod export;

#[derive(Debug)]
pub enum Instruction {
    Note(u8),
    Pause(f64),
    NoteOff,
    ChangeWave(u8),
    // Sweep { start: u8, end: u8, len: u32 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "urban_01";
    let filepath = format!("midi/{filename}.mid");
    let file = fs::read(filepath)?;
    let smf = Smf::parse(&file)?;
    let ppq = match smf.header.timing {
        midly::Timing::Metrical(t) => t.as_int() as f64,
        _ => unimplemented!("PPQ defined with Fps is currently unsupported."),
    };
    let mut glob_tics_to_secs = 0.0;
    // 500000 is the default tempo for midi, in case no tempo msg is sent at track start.
    calc_tick_to_secs(&mut glob_tics_to_secs, 500000, ppq);
    let tracks: Vec<_> = smf
        .tracks
        .iter()
        .map(|track| parse_track(track, &mut glob_tics_to_secs, ppq))
        .collect();
    fs::create_dir(format!("export/{filename}"))?;
    for (i, track) in tracks.iter().enumerate() {
        export_to_zup(&filename, &format!("track{i}"), &track)?;
    }
    Ok(())
}

fn parse_track(track: &Track, tick_to_secs: &mut f64, ppq: f64) -> Vec<Instruction> {
    use midly::MetaMessage;
    use midly::MidiMessage;
    use midly::TrackEventKind as TRK;

    let mut instructions: Vec<Instruction> = Vec::with_capacity(track.len());

    for &event in track {
        let mut secs_passed: f64 = 0.0;
        match event.kind {
            TRK::Midi { channel, message } => {
                let instruct = match message {
                    MidiMessage::NoteOff { .. } => Instruction::NoteOff,
                    MidiMessage::NoteOn { key, .. } => Instruction::Note(key.as_int()),
                    _ => continue,
                };
                let delta = event.delta.as_int() as f64 * *tick_to_secs;
                if delta == 0.0 {
                    let _ = instructions.pop();
                } else {
                    secs_passed += delta;
                    instructions.push(Instruction::Pause(delta));
                }
                instructions.push(instruct);
            }
            TRK::Meta(msg) => match msg {
                MetaMessage::Tempo(tempo) => {
                    println!("Tempo changed!");
                    calc_tick_to_secs(tick_to_secs, tempo.as_int(), ppq)
                }
                _ => {}
            },
            _ => {}
        }
    }
    instructions.shrink_to_fit();
    instructions
}

fn calc_tick_to_secs(tick_to_secs: &mut f64, tempo: u32, ppq: f64) {
    // tempo in microseconds/quarter div by ppq in ticks per quarter
    // and one million for microseconds to seconds
    *tick_to_secs = tempo as f64 / (ppq * 1_000_000.0);
}
