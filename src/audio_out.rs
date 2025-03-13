
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs
extern crate sdl2;

use sdl2::{audio::{AudioCallback, AudioSpecDesired}, AudioSubsystem};
use std::sync::mpsc::Receiver;
use std::fmt;

use crate::{util::get_freqy, midi::SoundCommand};


/**
 * Uses sdl2 to create an audio subsystem and a default desired audio spec
 * 
 * The audio spec defines how many samples per second are used when creating an audio
 * waveform
 */
pub fn init_audio_out(samples_per_second: Option<i32>)-> (AudioSubsystem, AudioSpecDesired) {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: samples_per_second, // default is usually 44_100
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    (audio_subsystem, desired_spec)
}


/**
 * Custom audio callback contains:
 * 
 * rx: Receiver, which will ingest any new SoundCommands. Recall that a sound command is a midi note with an attached frequency
 * 
 * currently_playing_waveforms: An array containing the midi notes currently being played
 * 
 */
#[derive(Debug)]
pub struct CustomAudioCallback {
    pub rx: Receiver<SoundCommand>,
    pub currently_playing_waveforms: Vec<u8>,
    pub freq: f32,
    pub phase: f32,
    pub volume: f32,
    pub spec_freq: i32,
}

impl fmt::Display for CustomAudioCallback {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "freq: {}, phase: {}, volume: {}, spec_freq: {}", self.freq, self.phase, self.volume, self.spec_freq)
  }
}

impl AudioCallback for CustomAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.receive();
        self.modify_buffer(out);

        println!("self: {}", self);
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
            self.currently_playing_waveforms.iter().for_each(|note| {
                let frequency = get_freqy(*note);

                let mut new_x: Vec<f32> = vec![];

                let mut sum = 0.;
                for x in out.iter_mut() {
                    // TODO: real time switching between the two
                    // *x = square_wave(self.phase, self.volume);
                    // self.phase = (self.phase + (self.freq / self.spec_freq as f32)) % 1.0;
                    // *x += solra_wave(self.phase, self.volume);

                    let x_val = *x + crate::audio_waves::solra_wave(self.phase, self.volume);
                    new_x.push(x_val);
                    self.phase += std::f32::consts::TAU * frequency / self.spec_freq as f32;

                    // sum gets used for finding 'norm'
                    sum += x_val.powf(2.);
                }
                
                // normalization logic https://www.reddit.com/r/learnrust/comments/16glmwa/comment/k08rsv2
                // todo: handle norm == 0
                let norm = sum.sqrt();

                for (i, x) in out.iter_mut().enumerate() {
                    *x = new_x[i] / norm;
                }

                // original, clean sounding wave
                // if we want to have a single voice. We can switch to this
                // for x in out.iter_mut() {
                //     *x =  crate::audio_waves::solra_wave(self.phase, self.volume);
                //     self.phase += std::f32::consts::TAU * frequency / self.spec_freq as f32;
                // }
            });
        }
    }

    fn handle_sound_command(&mut self, sound_command: SoundCommand) {
        self.phase = 0.;
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
