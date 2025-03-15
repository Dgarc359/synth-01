
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs
extern crate sdl2;

use sdl2::{audio::{AudioCallback, AudioSpecDesired}, AudioSubsystem};
use std::sync::mpsc::Receiver;
use std::fmt;

use crate::{audio_waves::sin_wave, midi::SoundCommand, util::get_freqy};


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
    pub phase_angle: f32,
    pub volume: f32,
    pub spec_freq: i32,
}

impl fmt::Display for CustomAudioCallback {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "freq: {}, phase_angle: {}, volume: {}, spec_freq: {}", self.freq, self.phase_angle, self.volume, self.spec_freq)
  }
}

impl AudioCallback for CustomAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.receive();
        self.modify_buffer(out);

        // println!("self: {}, first 5 outbuf: {:#?}", self, out[0..5].to_vec());
    }
}

impl CustomAudioCallback {
    fn receive(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            self.handle_sound_command(msg);
        }
    }

    fn modify_buffer(&mut self, buffer: &mut [f32]) {
        if self.currently_playing_waveforms.len() == 0 {
            for x in buffer.iter_mut() {
                *x = 0.0;
            }
        } else if self.currently_playing_waveforms.len() >= 1 {
            let out_clone = buffer.to_vec();
            let wave_coefficient = 0.2;
            let volume_bias = 0.5;

            let note_buffers: Vec<Vec<f32>> = self.currently_playing_waveforms
                .iter()
                .map(|note|  {
                    let mut new_x: Vec<f32> = vec![];
                    let frequency = get_freqy(*note);

                    for (i, x) in out_clone.iter().enumerate() {
                        // new_x.push(crate::audio_waves::sin_wave(self.phase_angle, self.volume));
                        // self.phase_angle = std::f32::consts::TAU * frequency / self.spec_freq as f32;
                        // self.phase_angle = self.phase_angle % std::f32::consts::TAU;

                        let phase_angle =  std::f32::consts::TAU * frequency * (i as f32 / self.spec_freq as f32);
                        new_x.push(phase_angle.sin() * wave_coefficient);
                    }

                    new_x
                })
                .collect();
            

            for x in buffer.iter_mut() {
                *x = 0.;
            }

            
            for (i, x) in buffer.iter_mut().enumerate() {
                for note_buffer in note_buffers.clone() {
                    *x += note_buffer[i]
                };

                // self.volume = (std::f32::consts::TAU * 0.2 * (i as f32/ 44_100.)).sin();
                // println!("volume: {}", self.volume);
                *x = (*x * self.volume) + volume_bias;
                self.freq = *x;
            }

            println!("out_buf|{:?}", buffer);
            // println!("out_buf size: {}", buffer.len())
        }
    }

    fn handle_sound_command(&mut self, sound_command: SoundCommand) {
        // self.phase_angle = 0.;
        // set internal frequencies and other values based on sound command
        match sound_command {
            SoundCommand::NoteOff { freq , midi_note } => {
                if let Some(index_of_note) = self.currently_playing_waveforms.iter().position(|&note| note == midi_note) {
                    // self.freq = (self.freq - freq).max(0.0_f32);
                    self.currently_playing_waveforms.remove(index_of_note);
                }

                // if self.currently_playing_waveforms.len() == 0 {
                //     self.volume = 0.0;
                // }
            }
            SoundCommand::NoteOn { freq, volume, midi_note } => {
                if self.currently_playing_waveforms.contains(&midi_note) {
                    // do nothing??
                } else {
                    // self.freq += freq;
                    // let vol_result = volume / 10_000.0_f32;
                    // self.volume = vol_result.max(0.5_f32).min(0.5_f32).max(0.0_f32);
                    // self.volume = vol_result;
                    self.volume = 1.;
                    // self.volume = 1.;
                    self.currently_playing_waveforms.push(midi_note);
                }
            }
        }

        // println!("finished handling sound command: {}", self.freq);
    }
}
