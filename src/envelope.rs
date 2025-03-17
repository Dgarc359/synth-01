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
    pub next_state: Option<AdsrEnvelopeStates>,
}

impl EnvelopeSingleStateConfig {
    pub fn new(state_type: AdsrEnvelopeStates, target: f32, duration: u16, next_state: Option<AdsrEnvelopeStates>, ) -> Self {
        Self {
            state_type, duration, target, next_state
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

    fn next(&self, current_config: EnvelopeSingleStateConfig, val: u16) -> u16 {
        match current_config.state_type {
            AdsrEnvelopeStates::Attack => { val.wrapping_add(1).min(current_config.duration) }
            AdsrEnvelopeStates::Delay => { val.wrapping_sub(1).max(current_config.duration) }
            AdsrEnvelopeStates::Sustain => { val }
            AdsrEnvelopeStates::Release => { val.wrapping_sub(1).max(current_config.duration) }
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
                        self.current_duration = self.next(current_config, self.current_duration);
                        self.current_val = crate::util::normalize(self.current_duration, current_config.duration, 0);

                        self.set_current_config_state(next_config);
                    } else {
                        // if we havent hit the target yet, then do the value next!
                        self.current_duration = self.next(current_config, self.current_duration);

                    }
                }
            }
        }
     }

    pub fn get_normalized_current_value(&self) -> f32 {
        if self.get_current_config_from_state().is_some() {
            self.current_val
        } else {
            1.
        }
    }

    pub fn new(current_duration: u16, start_val: f32, current_state: Option<AdsrEnvelopeStates>, state_config: AdsrEnvelopeConfig) -> Self {
        Self {
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
