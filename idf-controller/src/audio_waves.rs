pub fn square_wave(phase: f32, volume: f32) -> f32 {
    if phase <= 0.5 {
        -volume
    } else {
        volume
    }
}

pub fn sin_wave(phase: f32, volume: f32) -> f32 {
    return phase.sin() * volume;
}