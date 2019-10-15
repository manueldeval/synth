use crate::synth::dsp::audionode::AudioNode;
use crate::synth::utils::converters::voltage_to_zero_to_one;
use crate::synth::utils::converters::voltage_to_boolean;

/*
=========================================
Null
=========================================
*/

pub struct ADSRNode {
  attack_time: f32,
  decay_time: f32,
  decay_level: f32,
  sustain_time: f32,
  release_time: f32,
  note_on: bool,
  input_signal: f32,

  output_value: f32,

  last_amp: f32
}

impl ADSRNode {
  pub const INPUT_SIGNAL: i32 = 0;
  pub const ATTACK_TIME: i32 = 1;
  pub const DECAY_TIME: i32= 2;
  pub const DECAY_LEVEL: i32= 3;
  pub const SUSTAIN_TIME: i32= 4;
  pub const RELEASE_TIME: i32= 5;
  pub const TRIGGER: i32= 6;

  pub const OUTPUT_SIGNAL: i32 = 0;

  pub fn new() -> ADSRNode {
    ADSRNode {    
          attack_time: -0.9,
          decay_time: 0.0,
          decay_level: 0.0,
          sustain_time: 0.0,
          release_time: 0.0,
          note_on: false,
          input_signal: 0.0,

          output_value: 0.0,

          last_amp: 0.0
    }
  } 
}

impl AudioNode for ADSRNode { 
  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      ADSRNode::INPUT_SIGNAL  => self.input_signal = value,
      ADSRNode::ATTACK_TIME   => self.attack_time = voltage_to_zero_to_one(value),
      ADSRNode::DECAY_TIME    => self.decay_time = voltage_to_zero_to_one(value),
      ADSRNode::DECAY_LEVEL   => self.decay_level = voltage_to_zero_to_one(value),
      ADSRNode::SUSTAIN_TIME  => self.sustain_time = voltage_to_zero_to_one(value),
      ADSRNode::RELEASE_TIME  => self.release_time = voltage_to_zero_to_one(value),
      ADSRNode::TRIGGER       => self.note_on = voltage_to_boolean(value),
      _ => ()
    };
  }
  fn compute(&mut self) { 
    self.output_value = 0.0;
   }
  fn get_output_value(&self, ouput: i32) -> f32 { 
    match ouput {
      ADSRNode::OUTPUT_SIGNAL => self.output_value,
      _ => 0.0
    }
  }
}