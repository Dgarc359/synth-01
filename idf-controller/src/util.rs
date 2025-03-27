
pub fn normalize_f32(val: f32, max_value: f32, min_value: f32) ->f32 {
    val - min_value / max_value - min_value
}

pub fn normalize(val: u16, max_value: u16, min_value: u16) -> f32 {
    // println!("normalizing val: {}, max_val: {}, min_val: {}", val, max_value, min_value);
    (val.saturating_sub(min_value)) as f32 / (max_value.saturating_sub(min_value)) as f32
    // (val - min_value) / (max_value - min_value)
}

// given a number, return the frequency required for the note
pub fn get_freqy(i: u8) -> f32 {
    // https://en.wikipedia.org/wiki/Musical_note#MIDI
    return 2.0f32.powf((i as f32 - 69.0) / 12.0) * 440.0;
}

use chrono::prelude::{DateTime, Utc};

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}