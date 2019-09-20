use crate::synth::audionode::AudioNode;
use crate::synth::converters::voltage_to_frequency;
use crate::synth::converters::voltage_to_zero_to_one;
use crate::synth::converters::hard_clip;

// https://github.com/OpenBCI/OpenBCI_GUI/blob/master/OpenBCI_GUI/libraries/minim/src/ddf/minim/ugens/MoogFilter.java

pub struct MoogFilterNode {
  input: f32,
  lp_output: f32,
  hp_output: f32,
  bp_output: f32,
  coefs: [f32;5],
  sample_rate_hz: f32,
  cutoff_frequency_volt: f32,
  resonance_volt: f32
}

impl MoogFilterNode {

  pub const INPUT_AUDIO: i32 = 0;
  pub const INPUT_CUTOFF: i32 = 1;
  pub const INPUT_RESONANCE: i32 = 2;

  pub const OUPUT_LP: i32 = 0;
  pub const OUPUT_BP: i32 = 1;
  pub const OUPUT_HP: i32 = 2;

  pub fn new() -> MoogFilterNode {
    MoogFilterNode { 
      input: 0.0,
      lp_output: 0.0,
      hp_output: 0.0,
      bp_output: 0.0,
      coefs: [0.0;5],
      sample_rate_hz: 44_0000.0,
      cutoff_frequency_volt: 1.0,
      resonance_volt: -1.0
    }
  }
}

impl AudioNode for MoogFilterNode {

  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      MoogFilterNode::INPUT_AUDIO => self.input = value,
      MoogFilterNode::INPUT_CUTOFF => self.cutoff_frequency_volt = value,
      MoogFilterNode::INPUT_RESONANCE => self.resonance_volt = value,
      _ => ()
    };
  }

  fn configure(&mut self,frequency: i32) {
    self.sample_rate_hz = frequency as f32;
  }
  
  fn compute(&mut self) { 
    		// Set coefficients given frequency & resonance [0.0...1.0]

    // temporary buffers
		let mut t1 : f32; 
		let mut t2 : f32;

    let norm_freq = voltage_to_frequency(self.cutoff_frequency_volt) / (self.sample_rate_hz * 0.5);
		let rez = hard_clip( voltage_to_zero_to_one(self.resonance_volt), 0.0, 1.0 );

		let q = 1.0 - norm_freq;
		let p = norm_freq + 0.8 * norm_freq * q;
		let f = p + p - 1.0;
		let q = rez * ( 1.0 + 0.5 * q * ( 1.0 - q + 5.6 * q * q ) );

    //============================
    let mut input = hard_clip(self.input, -1.0, 1.0 ); // hard clip

		input -= q * self.coefs[4]; // feedback
		
    t1 = self.coefs[1];
		self.coefs[1] = ( input + self.coefs[0] ) * p - self.coefs[1] * f;

	  t2 = self.coefs[2];
		self.coefs[2] = ( self.coefs[1] + t1 ) * p - self.coefs[2] * f;

		t1 = self.coefs[3];
		self.coefs[3] = ( self.coefs[2] + t2 ) * p - self.coefs[3] * f;

		self.coefs[4] = ( self.coefs[3] + t1 ) * p - self.coefs[4] * f;
		self.coefs[4] = self.coefs[4] - self.coefs[4] * self.coefs[4] * self.coefs[4] * 0.166667; // clipping

    self.coefs[0] = input;

    self.hp_output = input - self.coefs[4];
    self.lp_output = self.coefs[4];
    self.bp_output = 3.0 * (self.coefs[3] - self.coefs[4]);
  }
  
  fn get_output_value(&self, ouput: i32) -> f32 {
    match ouput {
      MoogFilterNode::OUPUT_BP => self.bp_output,
      MoogFilterNode::OUPUT_HP => self.hp_output,
      MoogFilterNode::OUPUT_LP => self.lp_output,
      _ => 0.0
    }
  }
}

