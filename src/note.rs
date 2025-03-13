use crate::{util::get_freqy,SoundCommand};

// thx solra for the Note and note parsing code <3
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Note {
    On { channel: u8, volume: u8, note: u8 },
    Off { channel: u8, note: u8 },
}


impl Note {
    pub fn from_buffer(message: &[u8]) -> Option<Note> {
        if message.len() < 2 {
            return None;
        };
        let channel = message[0] & 15;
        let command = message[0] >> 4;
        println!("note: {}, channel: {}, command: {}", message[1], channel, command);
        match command {
            8 => Some(Note::Off {
                channel,
                note: message[1],
            }),
            9 if message.len() >= 3 => {
                if message[2] == 0 {
                    Some(Note::Off {
                        channel,
                        note: message[1],
                    })
                } else {
                    Some(Note::On {
                        channel,
                        note: message[1],
                        volume: message[2],
                    })
                }
            }
            _ => None,
        }
    }
}


