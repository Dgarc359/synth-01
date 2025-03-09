# Synth 01

An experiment in some very very simple digital signal processing

The goal of this project is to provide a currently undisclosed midi device
with a sound module deployed onto an esp32 and outputting audio data

# Key dependencies
- Midir: programmatic interfacing with midi devices
- sdl2: simple audio output solution

# Roadmap
- Loadable onto esp32 (kind of awkward since that backdoor dropped)
- Some kind of circuitry setup to take usb-c midi and output audio piped to a female 3.5 mm or 6mm audio jack
- MIDI thru on esp32 setup
