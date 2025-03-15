use crate::{util::get_freqy, note::Note};

/**
 * A sound command comes from a midi device
 */
#[derive(Debug)]
pub enum SoundCommand {
    NoteOn { midi_note: u8, freq: f32, volume: f32, phase_angle: f32 },
    NoteOff { midi_note: u8, freq: f32 },
}

#[derive(Debug)]
pub struct Wave {
    pub midi_note: u8, pub freq: f32, pub volume: f32, pub phase_angle: f32
}

impl Wave {

    pub fn increment_phase(&mut self, spec_freq: f32) {
        self.phase_angle += std::f32::consts::TAU * self.freq / spec_freq;
        self.phase_angle = self.phase_angle % std::f32::consts::TAU;
    }
}





impl SoundCommand {


   pub fn get_phase(&self) -> Option<f32>  {
    match *self {
        SoundCommand::NoteOn {phase_angle , ..} =>  { Some(phase_angle) },
        SoundCommand::NoteOff {.. } => { None }
    }
   } 


    pub fn from_note(note: Note) -> SoundCommand {
        match note {
            Note::On { note, volume, .. } => SoundCommand::NoteOn {
                midi_note: note,
                freq: get_freqy(note),
                volume: volume as f32,
                phase_angle: 0.,
            },
            Note::Off { note, .. } => SoundCommand::NoteOff {
                midi_note: note,
                freq: get_freqy(note),
            },
        }
    }
}