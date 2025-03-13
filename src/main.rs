use std::error::Error;
use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

use midir::{Ignore, MidiInput};

mod audio_out;
mod audio_in;
mod audio_waves;
mod note;
mod midi;
mod util;


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel::<crate::midi::SoundCommand>();

    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let Some((in_port, in_port_name)) = crate::audio_in::get_input_port(&midi_in) else {
        todo!()
    };

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            if let Some(parsed_note) = crate::note::Note::from_buffer(&message) {
                let sound_command = crate::midi::SoundCommand::from_note(parsed_note);

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

    let (audio_subsystem, desired_spec) = crate::audio_out::init_audio_out(Some(44_100));
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // yield this custom audio callback
            crate::audio_out::CustomAudioCallback {
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
