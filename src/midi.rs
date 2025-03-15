use crate::{util::get_freqy, note::Note};

/**
 * A sound command comes from a midi device
 */
#[derive(Debug)]
pub enum SoundCommand {
    NoteOn { midi_note: u8, freq: f32, volume: f32 },
    NoteOff { midi_note: u8, freq: f32 },
}

impl SoundCommand {
    pub fn from_note(note: Note) -> SoundCommand {
        match note {
            Note::On { note, volume, .. } => SoundCommand::NoteOn {
                midi_note: note,
                freq: get_freqy(note),
                volume: volume as f32,
            },
            Note::Off { note, .. } => SoundCommand::NoteOff {
                midi_note: note,
                freq: get_freqy(note),
            },
        }
    }
}