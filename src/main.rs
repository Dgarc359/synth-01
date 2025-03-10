use std::error::Error;
use std::io::stdin;

use midir::{Ignore, MidiInput};

mod audio_in;
use audio_in::get_input_port;

// given a number, return the frequency required for the note
pub fn get_freqy(i: u8) -> f32 {
    println!("got i: {:#?}", i);
    // https://en.wikipedia.org/wiki/Musical_note#MIDI
    return 2.0f32.powf((i as f32 - 69.0) / 12.0 ) * 440.0
}


// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs
extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

// thx solra for the Note and note parsing code <3
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Note {
    On { channel: u8, volume: u8, note: u8 },
    Off { channel: u8, note: u8 },
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

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let Some((in_port, in_port_name)) = get_input_port(&midi_in) else { todo!() };

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            if let Some(note) = parse_note_message(&message) {
                match note {
                    Note::On { note, .. } => {
                        println!("got note {:#?}", note);
                        let sdl_context = sdl2::init().unwrap();
                        let audio_subsystem = sdl_context.audio().unwrap();
                        let desired_spec = AudioSpecDesired {
                            freq: Some(44_100),
                            channels: Some(1), // mono
                            samples: None,     // default sample size
                        };
                        let device = audio_subsystem
                            .open_playback(None, &desired_spec, |spec| {
                                // Show obtained AudioSpec
                                println!("{:?}", spec);
                                let note_freq = get_freqy(note);
                                println!("got note frequency: {:#?}", note_freq);

                                // initialize the audio callback
                                SquareWave {
                                    //phase_inc: 440.0 / spec.freq as f32,
                                    phase_inc: note_freq / spec.freq as f32,
                                    phase: 0.0,
                                    volume: 0.25,
                                }
                            })
                            .unwrap();

                        let device_two = audio_subsystem
                            .open_playback(None, &desired_spec, |spec| {
                                let new_note = note + 3;
                                // Show obtained AudioSpec
                                println!("{:?}", spec);
                                let note_freq = get_freqy(new_note);
                                println!("got note frequency: {:#?}", note_freq);

                                // initialize the audio callback
                                SquareWave {
                                    //phase_inc: 440.0 / spec.freq as f32,
                                    phase_inc: note_freq / spec.freq as f32,
                                    phase: 0.0,
                                    volume: 0.25,
                                }
                            })
                            .unwrap();
                        device_two.resume();
                        device.resume();
                        std::thread::sleep(Duration::from_millis(10));
                        ()
                    }
                    Note::Off { .. } => { () }
                }
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
