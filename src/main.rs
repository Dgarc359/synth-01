use std::error::Error;
use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::time::Duration;

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

    let (audio_buf_tx, audio_buf_rx) = channel::<Vec<f32>>();

    let mut midi_in = MidiInput::new("midir reading input").expect("couldnt read midi inputs");
    midi_in.ignore(Ignore::None);

    println!("got midi input details, getting input port");

    // Get an input port (read from console if multiple are available)
    let Some((in_port, in_port_name)) = crate::audio_in::get_input_port(&midi_in) else {
        todo!()
    };

    println!("got midi input port, establishing conn");
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
    ).expect("failed to establish connection");

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    println!("setting up audio");

    let desired_spec = crate::audio_out::init_audio_out(Some(44_100));
    let audio_device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // yield this custom audio callback
            crate::audio_out::CustomAudioCallback {
                rx,
                tx: audio_buf_tx,
                currently_playing_waveforms: vec![],
                freq: 0.0,
                phase_angle: 0.0,
                volume: 0.0,
                spec_freq: spec.freq,
            }
        })
        .unwrap();

    println!("set up audio");

    println!("setting up video");
    let video_subsystem = sdl_context.video().unwrap();


    // todo: reduce availability
    let window = video_subsystem
        .window("synthesizer-debug-window", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    const WIDTH: usize = 255;
    const HEIGHT: usize = 255;
    const PITCH: usize = 4 * WIDTH;
    const RESOLUTION: usize = WIDTH * HEIGHT * 4;
    // create textures
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let pixels_as_u8: &mut [u8] = &mut [0; RESOLUTION];
    // TODO: safer way to calculate than this
    let center_y = HEIGHT / 2;
    println!("set up video");

    let mut event_pump = sdl_context.event_pump().unwrap();


    'running: loop {
        audio_device.resume();
        pixels_as_u8.fill(0);

        while let Ok(buf) = audio_buf_rx.try_recv() {
            let y = 0;
            for x in 0..WIDTH {
                if let val = buf[x] {
                    let new_val = center_y + (val * 50.) as usize;
                    // let i = (x * 4) + (buf[x] * PITCH);
                    let i = (x * 4) + (new_val * PITCH);
                    pixels_as_u8[i] = 0;
                    pixels_as_u8[1 + i] = 255;
                    pixels_as_u8[2 + i] = 255;
                    pixels_as_u8[3 + i] = 255;
                }
            }

            texture
                .update(None, pixels_as_u8, PITCH)
                .expect("couldnt copy raw pixels");

            canvas
                .copy(&texture, None, None)
                .expect("couldnt copy texture to canvas");

            println!("got buf: {:?}", buf)
        }
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } | Event::KeyDown { keycode: Some(Keycode::KpEnter), .. } => break 'running,
                _ => {}
            }
        }
        // input.clear();
        // if let _ = stdin().read_line(&mut input) { // wait for next enter key press
        //     break 'running;
        // }
    }

    Ok(())
}
