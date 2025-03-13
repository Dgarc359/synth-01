


// given a number, return the frequency required for the note
pub fn get_freqy(i: u8) -> f32 {
    // https://en.wikipedia.org/wiki/Musical_note#MIDI
    return 2.0f32.powf((i as f32 - 69.0) / 12.0) * 440.0;
}