use std::{error::Error, fs};

use midly::{Smf, Track};

enum Instruction {
    Note(u8),
    Pause(u8),
    ChangeWave(u8),
    // Sweep { start: u8, end: u8, len: u32 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "midi/test1.mid";
    let file = fs::read(filepath)?;
    let smf = Smf::parse(&file)?;
    let ppq = match smf.header.timing {
        midly::Timing::Metrical(t) => t.as_int() as f64,
        _ => unimplemented!("PPQ defined with Fps is currently unsupported."),
    };
    println!("{:#?}", smf);
    Ok(())
}

fn parse_track(track: &Track, ppq: f64) {
    use midly::MetaMessage;
    use midly::MidiMessage;
    use midly::TrackEventKind as TRK;

    let mut tick_to_secs = 0.0;
    let mut secs_passed = 0.0;
    let mut instructions: Vec<Instruction> = vec![];

    // 500000 is the default tempo for midi, in case no tempo msg is sent at track start.
    calc_tick_to_secs(&mut tick_to_secs, 500000, ppq);

    for &event in track {
        match event.kind {
            TRK::Midi { channel, message } => {
                let instruct = match message {
                    MidiMessage::NoteOff { key, vel: _ } => todo!(),
                    MidiMessage::NoteOn { key, vel: _ } => Instruction::Note(key.as_int()), // det här kommer inte funka (med glob time passed)
                    _ => continue,
                };
                instructions.push(instruct);
            }
            TRK::Meta(msg) => match msg {
                MetaMessage::Tempo(tempo) => {
                    calc_tick_to_secs(&mut tick_to_secs, tempo.as_int(), ppq)
                }
                _ => {}
            },
            _ => {}
        }
        secs_passed += event.delta.as_int() as f64 * tick_to_secs;
    }
}

fn calc_tick_to_secs(tick_to_secs: &mut f64, tempo: u32, ppq: f64) {
    // tempo in microseconds/quarter div by ppq in ticks per quarter
    // and one million for microseconds to seconds
    *tick_to_secs = tempo as f64 / (ppq * 1_000_000.0);
}
