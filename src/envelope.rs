#[derive(Debug, Clone, Copy)]
pub enum AdsrEnvelopeStates {
    Attack,
    Delay,
    Sustain,
    Release,
}

#[derive(Clone, Copy)]
pub struct EnvelopeSingleStateConfig { 
    pub state_type: AdsrEnvelopeStates,
    pub duration: u16, 
    pub target: f32, 
    pub start_value: f32,
    pub next_state: Option<AdsrEnvelopeStates>,
}

pub struct MinMax {
    min: f32,
    max: f32,
}

impl EnvelopeSingleStateConfig {
    pub fn new(state_type: AdsrEnvelopeStates, start_value: f32, target: f32, duration: u16, next_state: Option<AdsrEnvelopeStates>, ) -> Self {
        Self {
            state_type, duration, target, next_state, start_value
        }
    }

    /**
     * Get the minimum and maximum between the envelopes start value and the target value
     * the goal of this is to always have the min / max and not have to worry the user of the envelope
     * with calculating min / max for normalization
     * 
     * User should basically not worry about what the math behind normalization IS
     */
    pub fn get_min_max(&self) -> MinMax {
        MinMax {
            min: self.start_value.min(self.target),
            max: self.start_value.max(self.target),
        }
    }
}


#[derive(Clone, Copy)]
pub struct AdsrEnvelopeConfig {
    pub attack: EnvelopeSingleStateConfig,
    pub delay: EnvelopeSingleStateConfig,
    pub sustain: EnvelopeSingleStateConfig,
    pub release: EnvelopeSingleStateConfig,
}

#[derive(Clone, Copy)]
pub struct AdsrEnvelope {
    start_val: f32,
    current_val: f32,
    current_duration: u16,
    current_state: Option<AdsrEnvelopeStates>,
    state_config: AdsrEnvelopeConfig,
}

impl AdsrEnvelope {
    fn get_current_config_from_state(&self) -> Option<EnvelopeSingleStateConfig> {
        match self.current_state {
            Some(AdsrEnvelopeStates::Attack) => { Some(self.state_config.attack) }
            Some(AdsrEnvelopeStates::Delay) => { Some(self.state_config.delay) }
            Some(AdsrEnvelopeStates::Sustain) => { Some(self.state_config.sustain) }
            Some(AdsrEnvelopeStates::Release) => { Some(self.state_config.release) }
            None => { None }
        }
    }

    pub fn set_current_config_state(&mut self, state: Option<AdsrEnvelopeStates>) {
        self.current_state = state
    }

    pub fn get_current_envelope_state(&self) -> Option<AdsrEnvelopeStates> {
        self.current_state
    }

    fn next(&self, current_config: EnvelopeSingleStateConfig, val: f32) -> f32 {
        match current_config.state_type {
            AdsrEnvelopeStates::Attack => {
                val.wrapping_add(1).min(current_config.target)
            }
            AdsrEnvelopeStates::Delay => { val.wrapping_sub(1).max(current_config.target)}
            AdsrEnvelopeStates::Sustain => { val }
            AdsrEnvelopeStates::Release => { val.wrapping_sub(1).max(current_config.target) }
        }
    }


    //  each envelope state needs to eventually implement its own strategy for modifying the counter to hit its target 
    pub fn generate_next_value(&mut self) { 
        let current_config = self.get_current_config_from_state();

        if let Some(current_config) = current_config {
            match current_config.state_type {
                AdsrEnvelopeStates::Sustain {..} => {  },
                _ => { 
                    if self.current_val == current_config.target {
                        // we need to transition to the next state
                        let next_config = current_config.next_state.clone();
                        self.current_val = self.next(current_config, self.current_val);

                        self.set_current_config_state(next_config);
                    } else {
                        // if we havent hit the target yet, then do the value next!
                        self.current_val = self.next(current_config, self.current_val);

                    }
                }
            }
        } 
     }


    pub fn denormalize_current_value(&self) -> u16 {

    }

    pub fn get_normalized_current_value(&self) -> f32 {
        if let Some(current_config) = self.get_current_config_from_state() {
            let min_max = current_config.get_min_max();
            crate::util::normalize_f32(self.current_val, min_max.max, min_max.min)
        } else {
            1.
        }
    }

    pub fn new(current_duration: u16, start_val: f32, current_state: Option<AdsrEnvelopeStates>, state_config: AdsrEnvelopeConfig) -> Self {
        Self {
            start_val,
            state_config,
            current_state,
            current_val: start_val,
            current_duration,
        }
    }
}


impl AdsrEnvelopeConfig {
    pub fn new(attack: EnvelopeSingleStateConfig, delay: EnvelopeSingleStateConfig, sustain: EnvelopeSingleStateConfig, release: EnvelopeSingleStateConfig) -> Self {
        Self {
            attack,
            delay,
            sustain,
            release,
        }
    }

}
