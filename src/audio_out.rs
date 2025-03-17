// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs
extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::{ops::IndexMut, sync::mpsc::{Receiver, Sender}};
use std::fmt;

use crate::midi::{self, SoundCommand, Wave};

use crate::envelope::{AdsrEnvelope,AdsrEnvelopeStates, AdsrEnvelopeConfig, EnvelopeSingleStateConfig};


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


pub struct AudioOutput {
    pub id: u8,
    pub buf: Vec<f32>
}

/**
 * Custom audio callback contains:
 *
 * rx: Receiver, which will ingest any new SoundCommands. Recall that a sound command is a midi note with an attached frequency
 *
 * currently_playing_waveforms: An array containing the midi notes currently being played
 *
 */
pub struct CustomAudioCallback {
    // receive audio commmands
    pub rx: Receiver<SoundCommand>,
    // forward audio buffer for downstream consumers
    // ex: video ingesting audio buffer and displaying current wave
    pub tx: Sender<AudioOutput>,
    pub currently_playing_waveforms: Vec<midi::Wave>,
    pub current_master_volume: f32,
    pub max_master_volume: f32,
    pub spec_freq: i32,
}


impl fmt::Display for CustomAudioCallback {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "master_volume: {}, spec_freq: {}", self.current_master_volume, self.spec_freq)
  }
}

impl AudioCallback for CustomAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // println!("UNALTERED_AUDIO_BUFFER|{:?}",out);
        self.tx.send(AudioOutput { id: 255, buf: out.to_vec() }).unwrap();
        // println!("fresh|timestamp|{}|out_buf|{:?}", iso8601(&SystemTime::now()), out);
        self.receive();
        self.modify_buffer(out);
        // println!("modified|timestamp|{}|out_buf|{:?}", iso8601(&SystemTime::now()), out);

        // println!("\x1B[2J\x1B[1;1H");
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
        self.currently_playing_waveforms = self.currently_playing_waveforms
            .iter()
            .filter(|&&wave| wave.envelope.get_current_envelope_state().is_some())
            .cloned()
            .collect();
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
                .enumerate()
                .map(|(i, note)|  {
                    let mut wave: Vec<f32> = vec![];
                    // original, clean sounding wave
                    // if we want to have a single voice. We can switch to this

                    for (_, _) in out_clone.iter().enumerate() {
                        let envelope_coefficient = note.envelope.get_normalized_current_value();

                        println!("current state {:?}, coefficient: {}", note.envelope.get_current_envelope_state(), envelope_coefficient);

                        let sin_wave_sample =  envelope_coefficient * crate::audio_waves::sin_wave(note.phase_angle, note.volume);


                        wave.push(sin_wave_sample);

                        note.envelope.generate_next_value();
                        println!("After generating: current state {:?}, coefficient: {}", note.envelope.get_current_envelope_state(), envelope_coefficient);
                        println!("generated next value");
                        note.increment_phase(self.spec_freq as f32);
                    }

                    self.tx.send(AudioOutput{id:(i as u8).wrapping_mul(10),buf: wave.clone()}).unwrap();

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

                *x = *x * self.current_master_volume;
            }
        }
    }

    fn handle_sound_command(&mut self, sound_command: SoundCommand) {
        // set internal frequencies and other values based on sound command
        match sound_command {
            SoundCommand::Encode { midi_note, volume } => self.handle_encoders(midi_note, volume),
            SoundCommand::NoteOff { midi_note, .. } => self.handle_note_off(midi_note),
            SoundCommand::NoteOn { freq, midi_note , .. } => self.handle_note_on(freq, midi_note),
        }
    }


    fn create_adsr_envelope(&self) -> AdsrEnvelope {
        AdsrEnvelope::new(
            0,
            0.,
            Some(AdsrEnvelopeStates::Attack),
            AdsrEnvelopeConfig::new(
                EnvelopeSingleStateConfig::new(
                    AdsrEnvelopeStates::Attack,
                    1.,
                    300,
                    Some(AdsrEnvelopeStates::Delay),
                ),
                EnvelopeSingleStateConfig::new(
                    AdsrEnvelopeStates::Delay,
                    0.8,
                    100,
                    Some(AdsrEnvelopeStates::Sustain),
                ),
                EnvelopeSingleStateConfig::new(
                        AdsrEnvelopeStates::Sustain,
                        0.8,
                        255,
                        Some(AdsrEnvelopeStates::Release),
                ),
                EnvelopeSingleStateConfig::new(
                        AdsrEnvelopeStates::Release,
                        0.,
                        300,
                        None,
                ),
            )
        )
    }

    fn handle_note_on(&mut self, freq: f32, midi_note: u8) {
        let target_wave = self.currently_playing_waveforms.iter().position(|wave| {wave.midi_note == midi_note});

        match target_wave {
            None => {
                self.currently_playing_waveforms.push(Wave {
                    midi_note,
                    freq,
                    volume: self.current_master_volume,
                    phase_angle: 0.,

                    envelope: self.create_adsr_envelope(),
                });
            }
            Some(wave_idx) => {
                let _ = self.currently_playing_waveforms.index_mut(wave_idx);
            }
        }
    }

    fn handle_note_off(&mut self, midi_note: u8) {
        if let Some(index_of_note) = self.currently_playing_waveforms.iter().position(|wave| wave.midi_note == midi_note) {
            self.currently_playing_waveforms.index_mut(index_of_note).envelope.set_current_config_state(Some(AdsrEnvelopeStates::Release));
        }
    }

    fn handle_encoders(&mut self, midi_note: u8, volume: u8) {
        match midi_note {
            21 => {
                // println!("handling volume encoder");
                let normalized_volume = crate::util::normalize(volume as u16, 127, 0);

                self.current_master_volume = self.max_master_volume * normalized_volume;
                // println!("current vol {}", self.current_master_volume);
            }
            _ => todo!(),
        }
    }
}
