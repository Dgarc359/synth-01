
I'm writing this as a documentation of things to keep in mind about this problem space regarding audio.
This might contain information related to sdl2 as well, unless it seems necessary to separate the two

Important to keep in mind regarding audio programming:


with SDL2 rust, you make an audio callback function which receives an audio buffer. You need to create a wave
form via calculations and modify the buffer we have received with the wave form we've calculated

# Audio programming

Going from monophonic -> polyphonic

**level 1** An initial simple implementation is going to change the buffer to equal currently playing sound wave

While this is good to get started, it's not enough to push this to the next level. When you play two notes
at the same time, you'll have a lot of clipping as you'll jump between currently playing sound waves

**level 2** Track the phase angle for each individual note as you add it into the buffer

tracking the phase angle stops the wave from clipping to a new position between buffers. 

In terms of _audio programming_ this means that when we created our wave struct, we also now track a phase_angle
and oscillate it over time. Our phase angle is basically telling us how our sin wave oscillation is going to start

In level one, you would be updating the phase angle as you update the buffer. This stems from a naive implementation
where we don't even really have any kind of tracking for what frequencies are being played.

Now that we remove the clipping by tracking the phase angle, we can combine our frequencies by adding each individual amplitude value of a frequency together with each other


I was initially also going to add: create a new buffer, same size as the old buffer, for each individual wave, then add those buffers into the main buffer. This approach might have stemmed from some troubleshooting and might not be necessary. We might be able to add the values into the original buffer as we calculate them, avoiding the need for a second loop later on. I'll need to do some further research

**level 3** Envelope -- Attack, Decay, Sustain, Release

# Hardware interfacing

Interfacing with hardware. Current interface device is a novation launchkey mini

Only basic midi keys work. The next thing to add support for is mapping encoders on the launchkey to envelope / volume / wave shape.

each midi key has an integer value, ex: middle C -- 60
which can be mapped or calculated to their appropriate frequency

TODO: hardware BOM

novation launchkey -> esp32 (SOUND MODULE)

We'll need to worry about how audio gets sent out from an esp32. 
The novation will connect with the esp32 via USB. esp32 will run this synthesizer software, and have a physical
audio output outputting audio to a 6mm audio output (maybe even L / R outputs for stereo support eventually?)
which can be connected to some sort of amplifier

esp32 (CONFIGURATION MODULE) -> esp32 (SOUND MODULE)

The CONFIGURATION MODULE will communicate with the SOUND MODULE and help to enable wireless configuration
of the SOUND MODULE

??? -- Why??? In order to load up new samples or such things onto the synthesizer without connecting???

Maybe the configuration module allows us to extend to a greater set of encoders??? maybe the configuration module
is a wireless midi in???

THE TRUE PURPOSE OF THE CONFIGURATION MODULE IS YET TO BE DISCOVERED