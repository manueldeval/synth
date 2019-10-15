use crate::synth::dsp::audionode::AudioNode;
use crate::synth::utils::converters::voltage_to_zero_to_one;
use crate::synth::utils::converters::hard_clip;

/*
=========================================
Null
=========================================
*/

pub struct VCANode {
  mod_signal: f32,
  mod_amp: f32,
  mod_offset: f32,

  input_signal: f32,
  output_value: f32
}

impl VCANode {
  pub const MOD_SIGNAL: i32 = 0;
  pub const MOD_AMP: i32 = 1;
  pub const MOD_OFFSET: i32= 2;

  pub const INPUT_SIGNAL: i32 = 3;

  pub const OUTPUT_SIGNAL: i32 = 0;

  pub fn new() -> VCANode {
    VCANode {  mod_signal: 0.0, 
               mod_amp: 1.0,
               mod_offset: -1.0,

               input_signal: 0.0,
               output_value: 0.0 }
  } 
}

impl AudioNode for VCANode { 
  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      VCANode::MOD_SIGNAL => self.mod_signal = value,
      VCANode::MOD_AMP => self.mod_amp = voltage_to_zero_to_one(value),
      VCANode::MOD_OFFSET =>  self.mod_offset = voltage_to_zero_to_one(value),
      VCANode::INPUT_SIGNAL => self.input_signal = value,
      _ => ()
    };
  }
  fn compute(&mut self) { 
    let amp = self.mod_offset + (self.mod_signal * self.mod_amp);
    self.output_value = self.input_signal * hard_clip( amp ,0.0,1.0);
   }
  fn get_output_value(&self, ouput: i32) -> f32 { 
    match ouput {
      VCANode::OUTPUT_SIGNAL => self.output_value,
      _ => 0.0
    }
  }
}