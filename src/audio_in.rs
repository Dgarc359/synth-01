
use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput, MidiInputPort};

pub fn get_input_port(midi_in: &MidiInput) -> Option<(MidiInputPort,String)> {
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return None,
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            in_ports
                .get(input.trim().parse::<usize>().unwrap())
                .ok_or("invalid input port selected").unwrap()
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port).unwrap();

    Some((in_port.clone(), in_port_name))
}
