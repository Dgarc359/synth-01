use crate::{util::get_freqy, note::Note};
use std::fmt;

/**
 * A sound command comes from a midi device
 */
#[derive(Debug)]
pub enum SoundCommand {
    NoteOn { midi_note: u8, freq: f32, volume: f32, phase_angle: f32 },
    NoteOff { midi_note: u8, freq: f32 },
    Encode { midi_note: u8, volume: u8 }
}

#[derive(Debug, Clone, Copy)]
pub struct Wave {
    pub midi_note: u8, 
    pub freq: f32, 
    pub volume: f32, 
    pub phase_angle: f32, 
    // let's take attack in 'milliseconds' and just decrement
    // there might be a better value to use for this
    // I say 'milliseconds' because it's going to be a rough estimate
    pub current_attack: u16,
    pub min_attack: u16,
    pub max_attack: u16,

    pub current_release: u16,
    pub min_release: u16,
    pub max_release: u16,

    // TODO: is_decaying and other bools can be in a single int
    pub is_releasing: bool,

}

impl fmt::Display for Wave {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.freq)
  }
}

impl Wave {

    pub fn get_normalized_decay(&self) -> f32 {
        crate::util::normalize(self.current_release , self.max_release, self.min_release)
    }

    pub fn decrement_decay(&mut self) {
        self.current_release = self.current_release.saturating_sub(1);
    }

    pub fn get_normalized_attack(&self) -> f32 {
        crate::util::normalize(self.current_attack , self.max_attack, self.min_attack)
    }

    pub fn increment_attack(&mut self) {
        self.current_attack = self.current_attack.saturating_add(1).min(self.max_attack);
    }

    pub fn decrement_attack(&mut self) {
        self.current_attack = self.current_attack.saturating_sub(1);
    }

    pub fn increment_phase(&mut self, spec_freq: f32) {
        self.phase_angle += std::f32::consts::TAU * self.freq / spec_freq;
        self.phase_angle = self.phase_angle % std::f32::consts::TAU;
    }
}





impl SoundCommand {
    // I asked myself, why are we doing note -> sound command if it basically comes out to the same?
    // there's some casting, and also Note _technically_ comes from a midi device, where sound command
    // is relevant to our sdl 2 sound generation domain.
    // I don't think it's bad to keep both of those separate and then integrate??
    // it may lead to more bugs in the future, who knows.
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
            Note::Encoder { volume, note, .. } => SoundCommand::Encode {
                volume,
                midi_note: note,
            },
            _ => todo!(),
        }
    }
}