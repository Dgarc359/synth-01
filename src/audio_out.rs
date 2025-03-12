
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs
extern crate sdl2;

use sdl2::{audio::{AudioCallback, AudioSpecDesired}, AudioSubsystem};
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

/*
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
*/
