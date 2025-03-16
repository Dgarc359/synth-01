
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs
extern crate sdl2;

use sdl2::{audio::{AudioCallback, AudioSpecDesired}, AudioSubsystem};
use std::{ops::IndexMut, sync::mpsc::{Receiver, Sender}};
use std::fmt;

use crate::{audio_waves::sin_wave, midi::{self, SoundCommand, Wave}, util::get_freqy};

use chrono::prelude::{DateTime, Utc};
use std::time::{Duration, SystemTime};

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}


/**
 * Uses sdl2 to create an audio subsystem and a default desired audio spec
 * 
 * The audio spec defines how many samples per second are used when creating an audio
 * waveform
 */
pub fn init_audio_out(samples_per_second: Option<i32>)->AudioSpecDesired {
    let desired_spec = AudioSpecDesired {
        freq: samples_per_second, // default is usually 44_100
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    desired_spec
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
    // receive audio commmands
    pub rx: Receiver<SoundCommand>,
    // forward audio buffer for downstream consumers
    pub tx: Sender<Vec<f32>>,
    pub currently_playing_waveforms: Vec<midi::Wave>,
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
        // println!("UNALTERED_AUDIO_BUFFER|{:?}",out);
        self.tx.send(out.to_vec()).unwrap();
        // println!("fresh|timestamp|{}|out_buf|{:?}", iso8601(&SystemTime::now()), out);
        self.receive();
        self.modify_buffer(out);
        // println!("modified|timestamp|{}|out_buf|{:?}", iso8601(&SystemTime::now()), out);

        // println!("self: {}, first 5 outbuf: {:#?}", self, out[0..5].to_vec());
    }
}

impl CustomAudioCallback {
    fn receive(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            self.handle_sound_command(msg);
        }
    }


    // remove out from currently playing waveforms any waveforms that 
    // have fully decayed
    fn filter_out_decayed_waveforms(&mut self) {
        self.currently_playing_waveforms = self.currently_playing_waveforms.iter().filter(|&&wave| wave.current_decay != wave.min_decay).cloned().collect();
    }

    fn modify_buffer(&mut self, buffer: &mut [f32]) {
        self.filter_out_decayed_waveforms();


        if self.currently_playing_waveforms.len() == 0 {
            for x in buffer.iter_mut() {
                *x = 0.0;
            }
        } else if self.currently_playing_waveforms.len() >= 1 {
            let out_clone = buffer.to_vec();

            let waves: Vec<Vec<f32>> = self.currently_playing_waveforms
                .iter_mut()
                .map(|note|  {
                    let mut wave: Vec<f32> = vec![];
                    // original, clean sounding wave
                    // if we want to have a single voice. We can switch to this

                    for (_, _) in out_clone.iter().enumerate() {
                        let normalized_attack = note.get_normalized_attack();
                        let normalized_decay: f32 =  match note.is_decaying {
                            false => { 1. }
                            true => { note.get_normalized_decay() }
                        };


                        let sin_wave_sample = normalized_decay * normalized_attack * crate::audio_waves::sin_wave(note.phase_angle, note.volume);


                        wave.push(sin_wave_sample);
                        
                        note.increment_attack();

                        if note.is_decaying {
                            note.decrement_decay();
                        }

                        note.increment_phase(self.spec_freq as f32);
                    }

                    wave
                })
                .collect();
            

            for x in buffer.iter_mut() {
                *x = 0.;
            }

            
            for (i, x) in buffer.iter_mut().enumerate() {
                for wave in waves.clone() {
                    *x += wave[i]
                };

                // self.volume = (std::f32::consts::TAU * 0.2 * (i as f32/ 44_100.)).sin();
                // println!("volume: {}", self.volume);
                *x = *x * self.volume;
                self.freq = *x;
            }

            // println!("{:?}", buffer)
            // println!("out_buf size: {}", buffer.len())
        }
    }

    fn handle_sound_command(&mut self, sound_command: SoundCommand) {
        // self.phase_angle = 0.;
        // set internal frequencies and other values based on sound command
        match sound_command {
            SoundCommand::NoteOff { freq , midi_note } => {
                if let Some(index_of_note) = self.currently_playing_waveforms.iter().position(|note| note.midi_note == midi_note) {
                    // self.currently_playing_waveforms.remove(index_of_note);
                    self.currently_playing_waveforms.index_mut(index_of_note).is_decaying = true;
                }
            }
            SoundCommand::NoteOn { freq, volume, midi_note , .. } => {
                if self.currently_playing_waveforms.iter().find(|&wave| {
                    wave.midi_note == midi_note
                }).is_none() {
                    self.volume = 1.;
                    self.currently_playing_waveforms.push(Wave {
                        midi_note: midi_note,
                        freq: freq,
                        volume: 1.,
                        phase_angle: 0.,

                        current_attack: 0,
                        min_attack: 0,
                        max_attack: 300,

                        is_decaying: false,
                        current_decay: 300,
                        max_decay: 300,
                        min_decay: 0,
                    });
                }
            }
        }
    }
}
