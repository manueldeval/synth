use crate::synth::dsp::audionode::AudioNode;
use crate::synth::utils::converters::voltage_to_zero_to_one;

pub struct MixerNode {
  input1: f32,
  amp1: f32,
  input2: f32,
  amp2: f32,
  input3: f32,
  amp3: f32,
  input4: f32,
  amp4: f32,
  
  output_signal: f32,
  output_amp: f32
}

impl MixerNode {
  pub const INPUT1 : i32 = 0;
  pub const AMP1 : i32 = 1;
  pub const INPUT2 : i32 = 2;
  pub const AMP2 : i32 = 3;
  pub const INPUT3 : i32 = 4;
  pub const AMP3 : i32 = 5;
  pub const INPUT4 : i32 = 6;
  pub const AMP4 : i32 = 7;
  pub const OUTPUT_AMP : i32 = 8;


  pub const OUTPUT_SIGNAL : i32 = 0;

  pub fn new() -> MixerNode {
    MixerNode {
    input1: 0.0,
    amp1: 1.0,
    input2: 0.0,
    amp2: 1.0,
    input3: 0.0,
    amp3: 1.0,
    input4: 0.0,
    amp4: 1.0,
    output_amp: 1.0,
    output_signal: 0.0
    }
  }
}

impl AudioNode for MixerNode {

  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      MixerNode::INPUT1 => self.input1 = value,
      MixerNode::INPUT2 => self.input2 = value,
      MixerNode::INPUT3 => self.input3 = value,
      MixerNode::INPUT4 => self.input4 = value,
      
      MixerNode::AMP1 => self.amp1 = voltage_to_zero_to_one(value),
      MixerNode::AMP2 => self.amp2 = voltage_to_zero_to_one(value),
      MixerNode::AMP3 => self.amp3 = voltage_to_zero_to_one(value),
      MixerNode::AMP4 => self.amp4 = voltage_to_zero_to_one(value),
      MixerNode::OUTPUT_AMP => self.output_amp = voltage_to_zero_to_one(value),

      _ => ()
    }
  }

  fn compute(&mut self) {
    self.output_signal = (self.input1*self.amp1 + 
                          self.input2*self.amp2 + 
                          self.input3*self.amp3 +
                          self.input4*self.amp4) * self.output_amp;
    
  }

  fn get_output_value(&self,ouput: i32) -> f32 {
    match ouput {
      MixerNode::OUTPUT_SIGNAL => self.output_signal,
      _ => 0.0
    }
  }
}
