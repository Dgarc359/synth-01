# Synth 01

An experiment in some very very simple digital signal processing

The goal of this project is to provide a novation launchkey mini
with a sound module deployed onto an esp32 and outputting audio data
in order to have a portable sound module that can be reprogrammed on the fly

# Key dependencies
- Midir: programmatic interfacing with midi devices
- sdl2: simple audio output solution

# Short term roadmap
- Loadable onto esp32 (kind of awkward since that backdoor dropped)
- Some kind of circuitry setup to take usb-c midi and output audio piped to a female 3.5 mm or 6mm audio jack

# Long term Roadmap
- MIDI thru on esp32 setup

# References
- https://en.wikipedia.org/wiki/Musical_note#MIDI

# Troubleshooting

#### (windows) Compatibility with CMake < 3.5 has been remove from CMake

This error seems possible with CMake version 4.0.0-rc3 (AT LEAST) installed on
a windows system with their msi installer method.

> [!TIP]
> Install a `3.x` version of CMake on your windows system
>
> Known working [version](https://github.com/Kitware/CMake/releases/tag/v3.31.5)
