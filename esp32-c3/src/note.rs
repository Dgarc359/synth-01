// thx solra for the Note and note parsing code <3
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Note {
    // it might be the same as On, but we can match to ENCODER and have different behavior
    Encoder { channel: u8, volume: u8, note: u8 },
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
        // println!("note: {}, volume: {}, channel: {}, command: {},  full message: {:?}", message[1], message[2], channel, command, message);
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
            11 => {
                // note 21 -> 28 with command 11 are the encoders for our midi device
                Some(Note::Encoder {
                    channel,
                    note: message[1],
                    volume: message[2],
                })
            }
            _ => None,
        }
    }
}


