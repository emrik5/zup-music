use std::{
    fs,
    io::{Result, Write},
};

use crate::Instruction;

const TUNING: f64 = 440.0;

pub fn export_to_zup(dirname: &str, filename: &str, track: &[Instruction]) -> Result<()> {
    let output: Vec<String> = track
        .iter()
        .map(|instruction| match &instruction {
            Instruction::Note(note) => format!("F{:.3}", midi_to_freq(*note)),
            Instruction::Pause(pause) => format!("P{}", pause),
            Instruction::NoteOff => format!("F0.001"),
            Instruction::ChangeWave(_) => todo!(),
        })
        .collect();
    let output = output.join("\n");
    let mut file = fs::File::create(format!("export/{}/{}.zup", dirname, filename))?;
    file.write_all(&[&output.as_bytes(), "\nR".as_bytes()].concat())?;
    Ok(())
}
fn midi_to_freq(note: u8) -> f64 {
    // https://newt.phys.unsw.edu.au/jw/notes.html
    TUNING * ((note as f64 - 69.0) / 12 as f64).exp2()
}
