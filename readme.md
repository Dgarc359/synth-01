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
- Separate support for TRS vs DIN midi connections
- Different available chassis (plastic / metal / wood / epoxy)
- micro sd
- Out of the box presets + easy to create new presets

# References

> - https://en.wikipedia.org/wiki/Musical_note#MIDI
> - [Envelope](<https://en.wikipedia.org/wiki/Envelope_(music)#:~:text=In sound and music%2C an,sustain and release (ADSR).>)
> - https://en.wikipedia.org/wiki/MIDI#Electrical_specifications
> - [Not really all you need to know about TRS midi](https://www.morningstar.io/post/all-you-need-to-know-about-trs-midi-connections)
> - [MIDI for the Arduino - Build a MIDI Input Circuit](https://www.youtube.com/watch?v=GxfHijjn0ZM)
> - [TEENSY-Synth PART 9: MIDI INPUT](https://www.youtube.com/watch?v=l34CNfwfuIY)


# Troubleshooting

#### (windows) Compatibility with CMake < 3.5 has been remove from CMake

This error seems possible with CMake version 4.0.0-rc3 (AT LEAST) installed on
a windows system with their msi installer method.

> [!TIP]
> Install a `3.x` version of CMake on your windows system
>
> Known working [version](https://github.com/Kitware/CMake/releases/tag/v3.31.5)
