use crate::synth::audionode::AudioNode;
use crate::synth::converters::voltage_to_boolean;

pub struct SampleHoldNode {
  input_trigger: bool,
  input_last_trigger: bool,
  must_hold: bool,
  input_signal: f32,
  output_signal: f32
}

impl SampleHoldNode {
  pub const INPUT_TRIGGER : i32 = 0; 
  pub const INPUT_SIGNAL : i32 = 1;

  pub const OUTPUT_SIGNAL : i32 = 0;

  pub fn new() -> SampleHoldNode {
    SampleHoldNode {
      input_trigger: false, 
      input_last_trigger: false,
      input_signal: 0.0, 
      output_signal: 0.0,
      must_hold: false
    }
  }
}

impl AudioNode for SampleHoldNode {

  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      SampleHoldNode::INPUT_SIGNAL => self.input_signal = value,
      SampleHoldNode::INPUT_TRIGGER => {
        self.input_last_trigger = self.input_trigger;
        self.input_trigger = voltage_to_boolean(value);
        self.must_hold = self.input_trigger && !self.input_last_trigger;
      },
      _ => ()
    }
  }

  fn compute(&mut self) {
    if self.must_hold {
      self.output_signal = self.input_signal;
    }
  }

  fn get_output_value(&self,ouput: i32) -> f32 {
    match ouput {
      SampleHoldNode::OUTPUT_SIGNAL => self.output_signal,
      _ => 0.0
    }
  }
}
