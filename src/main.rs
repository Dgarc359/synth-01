use std::error::Error;
use std::io::stdin;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

use midir::{Ignore, MidiInput};

mod audio_in;
use audio_in::get_input_port;

mod audio_out;

// given a number, return the frequency required for the note
pub fn get_freqy(i: u8) -> f32 {
    println!("got i: {:#?}", i);
    // https://en.wikipedia.org/wiki/Musical_note#MIDI
    return 2.0f32.powf((i as f32 - 69.0) / 12.0 ) * 440.0
}

// thx solra for the Note and note parsing code <3
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Note {
    On { channel: u8, volume: u8, note: u8 },
    Off { channel: u8, note: u8 },
}

fn note_to_sound_command(note: Note) -> SoundCommand {
    match note {
        Note::On { note, volume, .. } => {
            SoundCommand::NoteOn { freq: get_freqy(note), volume: volume as f32 }
        },
        Note::Off { note, .. } => { SoundCommand::NoteOff { freq: get_freqy(note) } },
    }
}

fn parse_note_message(message: &[u8]) -> Option<Note> {
    if message.len() < 2 {
        return None;
    };
    let channel = message[0] & 15;
    let command = message[0] >> 4;
    println!("channel: {}, command: {}", channel, command);
    match command {
        8 => Some(Note::Off {
            channel,
            note: message[1],
        }),
        9 if message.len() >= 3 => {
            //println!("handling command 9 with message: {:#?}", message);
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

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

#[derive(Debug)]
enum SoundCommand {
    NoteOn { freq: f32, volume: f32 },
    NoteOff { freq: f32 },
}

#[derive(Debug)]
struct CustomAudioCallback {
    rx: Receiver<SoundCommand>,
    freq: f32,
    phase: f32,
    volume: f32,
}

impl CustomAudioCallback {
    // NOTE: THIS WILL BLOCK INFINITELY
    pub fn receive(&mut self) {
        while let Ok(msg) = self.rx.recv() {
            self.handle_sound_command(msg);
        }
    }

    fn handle_sound_command(&mut self, sound_command: SoundCommand) {

      // set internal frequencies and other values based on sound command
      match sound_command {
        SoundCommand::NoteOff { freq} => {
          if self.freq == freq {
            self.volume = 0.0
          }
         },
        SoundCommand::NoteOn { freq, volume} => {
          self.freq = freq;
          self.volume = volume;
        },
      }


      // fill buffer with sounds??????
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel::<SoundCommand>();

    let mut input = String::new();
    let mut audio_callback = CustomAudioCallback {
        rx,
        freq: 0.0,
        phase: 0.0,
        volume: 0.0,
    };

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let Some((in_port, in_port_name)) = get_input_port(&midi_in) else { todo!() };

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            if let Some(parsed_note) = parse_note_message(&message) {
                let sound_command = note_to_sound_command(parsed_note);

                let tx = tx.clone();

                thread::spawn(move || {
                    tx.send(sound_command).unwrap();
                });
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    audio_callback.receive();
    /*
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    */
    println!("Closing connection");
    Ok(())
}
