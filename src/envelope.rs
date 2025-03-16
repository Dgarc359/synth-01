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
    pub length: u16, 
    pub target: u16, 
    pub next_state: Option<AdsrEnvelopeStates>,
}



impl EnvelopeSingleStateConfig {
    pub fn new(state_type: AdsrEnvelopeStates, length: u16, target: u16, next_state: Option<AdsrEnvelopeStates>) -> Self {
        Self {
            state_type, length, target, next_state,
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
    start_val: u16,
    current_val: u16,
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

    pub fn next(&self, current_config: EnvelopeSingleStateConfig, val: u16) -> u16 {

        // 1
        todo!()
    }

    fn attack_next_function(val: u16) -> u16 {
        todo!()
    }


    //  each envelope state needs to eventually implement its own strategy for modifying the counter to hit its target 
    pub fn generate_next_value(&mut self) -> u16 { 
        if let Some(current_config) = self.get_current_config_from_state() {
            // if we havent hit the target yet, then 
            if self.current_val == current_config.target {
                // we need to transition to the next state
                // self.current_state = self.current_state.next_state
                let next_config = current_config.next_state.clone();
                self.current_val = self.next(current_config, self.current_val);

                self.current_state = next_config;
            }
        }

        self.current_val
     }

    pub fn get_normalized_value(&self) -> f32 {
    //    todo!()

        1.
    }

    pub fn new(start_val: u16, current_state: Option<AdsrEnvelopeStates>, state_config: AdsrEnvelopeConfig) -> Self {
        Self {
            start_val,
            state_config,
            current_state,
            current_val: start_val,
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
