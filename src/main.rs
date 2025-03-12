use std::error::Error;
use std::io::stdin;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::fmt;

use midir::{Ignore, MidiInput};

mod audio_in;
use audio_in::get_input_port;

mod audio_out;
use audio_out::init_audio_out;

extern crate sdl2;

use sdl2::audio::AudioCallback;

// given a number, return the frequency required for the note
pub fn get_freqy(i: u8) -> f32 {
    // https://en.wikipedia.org/wiki/Musical_note#MIDI
    return 2.0f32.powf((i as f32 - 69.0) / 12.0) * 440.0;
}

// thx solra for the Note and note parsing code <3
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Note {
    On { channel: u8, volume: u8, note: u8 },
    Off { channel: u8, note: u8 },
}

fn note_to_sound_command(note: Note) -> SoundCommand {
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

fn parse_note_message(message: &[u8]) -> Option<Note> {
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

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

#[derive(Debug)]
enum SoundCommand {
    NoteOn { midi_note: u8, freq: f32, volume: f32 },
    NoteOff { midi_note: u8, freq: f32 },
}

#[derive(Debug)]
struct CustomAudioCallback {
    rx: Receiver<SoundCommand>,
    currently_playing_waveforms: Vec<u8>,
    freq: f32,
    phase: f32,
    volume: f32,
    spec_freq: i32,
}

impl fmt::Display for CustomAudioCallback {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "freq: {}, phase: {}, volume: {}, spec_freq: {}", self.freq, self.phase, self.volume, self.spec_freq)
  }
}

impl CustomAudioCallback {
    fn receive(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            self.handle_sound_command(msg);
        }
    }

    fn modify_buffer(&mut self, out: &mut [f32]) {
        if self.currently_playing_waveforms.len() == 0 {
            for x in out.iter_mut() {
                *x = 0.0;
            }
        } else if self.currently_playing_waveforms.len() >= 1 {
            let frequencies: Vec<f32> = self.currently_playing_waveforms.iter().map(|note| {
                return get_freqy(*note)
            }).collect();
            
            for frequency in frequencies.iter() {
                let starter_phase = self.phase;
                let mut new_x: Vec<f32> = vec![];

                for x in out.iter_mut() {
                    // TODO: real time switching between the two
                    // *x = square_wave(self.phase, self.volume);
                    // self.phase = (self.phase + (self.freq / self.spec_freq as f32)) % 1.0;
                    // *x += solra_wave(self.phase, self.volume);
                    let x_val = *x + solra_wave(self.phase, self.volume);
                    new_x.push(x_val);
                    self.phase += std::f32::consts::TAU * frequency / self.spec_freq as f32;
                }
                
                // normalization logic https://www.reddit.com/r/learnrust/comments/16glmwa/comment/k08rsv2
                let norm = new_x
                    .iter()
                    .fold(0., |sum, &num| sum + num.powf(2.0))
                    .sqrt();
                // todo: handle norm == 0
                new_x = new_x.iter().map(|&b| b / norm).collect();

                for (i, x) in out.iter_mut().enumerate() {
                    *x = new_x[i];
                }

                self.phase = starter_phase;
            }
        }
    }

    fn handle_sound_command(&mut self, sound_command: SoundCommand) {
        // set internal frequencies and other values based on sound command
        match sound_command {
            SoundCommand::NoteOff { freq , midi_note } => {
                if let Some(index_of_note) = self.currently_playing_waveforms.iter().position(|&note| note == midi_note) {
                    self.freq = (self.freq - freq).max(0.0_f32);
                    self.currently_playing_waveforms.remove(index_of_note);
                }

                if self.currently_playing_waveforms.len() == 0 {
                    self.volume = 0.0;
                }
            }
            SoundCommand::NoteOn { freq, volume, midi_note } => {
                if self.currently_playing_waveforms.contains(&midi_note) {
                    // do nothing??
                } else {
                    self.freq += freq;
                    let vol_result = volume / 10_000.0_f32;
                    self.volume = vol_result.max(0.01_f32).min(0.02_f32).max(0.0_f32);
                    self.currently_playing_waveforms.push(midi_note);
                }
            }
        }

        println!("finished handling sound command: {}", self.freq);
    }
}

fn square_wave(phase: f32, volume: f32) -> f32 {
    if phase <= 0.5 {
        -volume
    } else {
        volume
    }
}

fn solra_wave(phase: f32, volume: f32) -> f32 {
    return phase.sin() * volume;
}

impl AudioCallback for CustomAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.receive();
        self.modify_buffer(out);

        println!("self: {}", self);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel::<SoundCommand>();

    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let Some((in_port, in_port_name)) = get_input_port(&midi_in) else {
        todo!()
    };

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

    let (audio_subsystem, desired_spec) = init_audio_out(Some(44_100));
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // yield this custom audio callback
            CustomAudioCallback {
                rx,
                currently_playing_waveforms: vec![],
                freq: 0.0,
                phase: 0.0,
                volume: 0.0,
                spec_freq: spec.freq,
            }
        })
        .unwrap();

    loop {
        device.resume();
        input.clear();
        let _ = stdin().read_line(&mut input); // wait for next enter key press
        break;
    }

    Ok(())
}
