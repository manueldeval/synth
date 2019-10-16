use crate::synth::dsp::audionode::AudioNode;
use crate::synth::utils::converters::voltage_to_zero_to_one;
use crate::synth::utils::converters::voltage_to_boolean;

use std::f32::consts::E;
/*
=========================================
http://www.earlevel.com/main/2013/06/03/envelope-generators-adsr-code/
=========================================
*/

enum EnvState {
  Idle,
  Attack,
  Decay,
  Sustain,
  Release
}


pub struct ADSRNode {
  state: EnvState,
  sample_rate: f32,
  attack_time: f32,
  decay_time: f32,
  sustain_level: f32,
  release_time: f32,
  target_ratio_A: f32,
  target_ratio_DR: f32,

  note_on: bool,
  input_signal: f32,

  output_value: f32,

  // Computed
  attack_coef: f32,
  attack_base: f32,

  decay_coef: f32,
  decay_base: f32,

  release_coef: f32,
  release_base: f32,
}

impl ADSRNode {
  pub const INPUT_SIGNAL: i32 = 0;
  pub const ATTACK_TIME: i32 = 1;
  pub const DECAY_TIME: i32= 2;
  pub const SUSTAIN_LEVEL: i32= 3;
  pub const SUSTAIN_TIME: i32= 4;
  pub const ATTACK_RATIO: i32 = 5;
  pub const DECAY_RELEASE_RATIO: i32 = 6;

  pub const TRIGGER: i32= 7;

  pub const OUTPUT_SIGNAL: i32 = 0;

  pub fn new() -> ADSRNode {
    ADSRNode {    
          state: EnvState::Idle,
          sample_rate: 44_000.0,
          
          attack_time: -0.9,
          decay_time: 0.0,
          sustain_level: 0.0,
          release_time: 0.0,
          note_on: false,
          input_signal: 0.0,

          output_value: 0.0,
          attack_base: 0.0,
          attack_coef: 0.0,
          target_ratio_A: 0.3,
          decay_coef: 0.0,
          decay_base: 0.0,
          target_ratio_DR: 0.0001,
          release_coef: 0.0,
          release_base: 0.0,
    }
  } 

  const MAX_ATTACK_TIME: f32 = 5.0;
  const MAX_DECAY_TIME: f32 = 10.0; 
  const MAX_RELEASE_TIME: f32 = 10.0; 

  fn calc_coef(&self, rate: f32, target_ratio: f32) -> f32 {
    return if rate <= 0.0 {
      0.0
    } else {
      E.powf( ((1.0 + target_ratio)/target_ratio).ln()) / rate
    }
  }

  fn set_attack_rate(&mut self, rate: f32) {
    self.attack_time = rate * self.sample_rate * ADSRNode::MAX_ATTACK_TIME;
    self.attack_coef = self.calc_coef(rate, self.target_ratio_A);
    self.attack_base = (1.0 + self.target_ratio_A) * (1.0 - self.attack_coef);
  }

  fn set_decay_rate(&mut self, rate: f32) {
    self.decay_time = rate * self.sample_rate * ADSRNode::MAX_DECAY_TIME;
    self.decay_coef = self.calc_coef(rate, self.target_ratio_DR);
    self.decay_base = (self.sustain_level - self.target_ratio_DR) * (1.0 - self.decay_coef);
  }

  fn set_release_rate(&mut self, rate: f32) {
    self.release_time = rate * self.sample_rate * ADSRNode::MAX_RELEASE_TIME;
    self.release_coef = self.calc_coef(rate, self.target_ratio_DR);
    self.release_base = - self.target_ratio_DR * (1.0 - self.release_coef);
  }

  fn set_sustain_level (&mut self, level: f32) {
    self.sustain_level = level;
    self.decay_base = (self.sustain_level - self.target_ratio_DR) * (1.0 - self.decay_coef);
  }

  fn set_target_ratio_A(&mut self,targetRatio: f32) {
    self.target_ratio_A = if targetRatio < 0.000000001 {
      0.000000001 // -180 dB
    } else {
      targetRatio
    };
    self.attack_coef = self.calc_coef(self.attack_time, self.target_ratio_A);
    self.attack_base = (1.0 + self.target_ratio_A) * (1.0 - self.attack_coef);
  }

  fn set_target_ratio_DR(&mut self, targetRatio: f32) {
    self.target_ratio_DR = if targetRatio < 0.000000001 {
      0.000000001 // -180 dB
    } else {
      targetRatio
    };
    self.decay_coef = self.calc_coef(self.decay_time, self.target_ratio_DR);
    self.release_coef = self.calc_coef(self.release_time, self.target_ratio_DR);
    self.decay_base = (self.sustain_level - self.target_ratio_DR) * (1.0 - self.decay_coef);
    self.release_base = - self.target_ratio_DR * (1.0 - self.release_coef);
  }

  fn reset(&mut self) {
    self.state = EnvState::Idle;
    self.output_value = 0.0;
  }

  fn gate(&mut self, gate: bool){
    match (gate,self.note_on){
      (true,false)    => self.state = EnvState::Attack, // New note_on
      (false,true)    => self.state = EnvState::Release, // Note off
      (_,_)           => ()
    };
    self.note_on = gate;
  }
}

impl AudioNode for ADSRNode { 

  fn set_sample_rate(&mut self,frequency: i32){
    self.sample_rate = frequency as f32;
  }

  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      ADSRNode::INPUT_SIGNAL  => self.input_signal = value,
      ADSRNode::ATTACK_TIME   => self.set_attack_rate(voltage_to_zero_to_one(value)),
      ADSRNode::DECAY_TIME    => self.set_decay_rate(voltage_to_zero_to_one(value)),
      ADSRNode::SUSTAIN_LEVEL => self.set_sustain_level(voltage_to_zero_to_one(value)),
      ADSRNode::ATTACK_RATIO  => self.set_target_ratio_A(voltage_to_zero_to_one(value)),
      ADSRNode::DECAY_RELEASE_RATIO => self.set_target_ratio_DR(voltage_to_zero_to_one(value)),
      ADSRNode::TRIGGER       => self.note_on = voltage_to_boolean(value),
      _ => ()
    };
  }

  fn compute(&mut self) { 
    match self.state {
          EnvState::Idle => (),
          EnvState::Attack => {
            self.output_value = self.attack_base + self.output_value * self.attack_coef;
            if self.output_value >= 1.0 {
              self.output_value = 1.0;
              self.state = EnvState::Decay;
            }
          }
          EnvState::Decay => {
            self.output_value = self.decay_base + self.output_value * self.decay_coef;
            if self.output_value <= self.sustain_level {
              self.output_value = self.sustain_level;
              self.state = EnvState::Sustain;
            }
          }
          EnvState::Sustain => (),
          EnvState::Release => {
            self.output_value  = self.release_base + self.output_value * self.release_coef;
            if self.output_value <= 0.0 {
              self.output_value = 0.0;
              self.state = EnvState::Idle;
            }
          }
    }
  }
  fn get_output_value(&self, ouput: i32) -> f32 { 
    match ouput {
      ADSRNode::OUTPUT_SIGNAL => self.output_value,
      _ => 0.0
    }
  }
}