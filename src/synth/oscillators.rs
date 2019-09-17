
use super::audionode::AudioNode;
use super::baseoscillator::BaseOscillator;
use super::baseoscillator::CommonOscillator;
use super::baseoscillator::OscillatorMode;
use super::converters::voltage_to_zero_to_one;

// ==================================
// SIN
// ==================================
pub struct SinNode {
  common_oscillator: CommonOscillator
}

impl SinNode{ 
  pub fn new(osc_mode: OscillatorMode,osc_frequency: f32, amp: f32,is_on: bool) -> SinNode {
    SinNode {
      common_oscillator: CommonOscillator::new(osc_mode,osc_frequency,amp,is_on)
    }
  }
}

impl BaseOscillator for SinNode {
  fn get_common_oscillator(&mut self) -> &mut CommonOscillator {
    &mut (self.common_oscillator)
  }

  fn compute_extended(&mut self){
    let radian = self.common_oscillator.sample_clock * self.common_oscillator.oscillator_frequency_hz * 2.0 * 3.141592 / self.common_oscillator.sample_rate;
    self.common_oscillator.value = radian.sin() * self.common_oscillator.oscillator_amp;
  }
}
//==================================
// SQUARE
//==================================
pub struct SquareNode {
  common_oscillator:  CommonOscillator,
  ratio: f32
}

impl SquareNode { 
  pub const INPUT_RATIO: i32 = 3;

  pub fn new(osc_mode: OscillatorMode,osc_frequency: f32, amp: f32,is_on: bool) -> SquareNode {
    SquareNode {
      common_oscillator: CommonOscillator::new(osc_mode,osc_frequency,amp,is_on),
      ratio: -1.0
    }
  }
}

impl BaseOscillator for SquareNode {
  
  fn get_common_oscillator(&mut self) -> &mut CommonOscillator {
    &mut (self.common_oscillator)
  }

  fn set_input_value_extended(&mut self, input: i32, value: f32){
    match input {
      SquareNode::INPUT_RATIO => self.ratio = voltage_to_zero_to_one(value) * 0.95, // 1 is a null signal (we stop a 90%)
      _ => ()
    };
  }

  fn compute_extended(&mut self){
    let ratio = self.ratio;
    let nbr_samples_per_period = self.common_oscillator.sample_rate/self.common_oscillator.oscillator_frequency_hz;
    self.common_oscillator.value = (if self.common_oscillator.sample_clock > (nbr_samples_per_period/(2.0 - ratio))  { 1.0 } else { -1.0 }) * self.common_oscillator.oscillator_amp;
  }
}


//==================================
// SAW
//==================================
pub struct SawNode {
  common_oscillator:  CommonOscillator,
  ratio: f32
}

impl SawNode { 
  pub const INPUT_RATIO: i32 = 3;

  pub const SAW_DOWN: f32 = 1.0;
  pub const SAW_UP: f32 = -1.0;
  pub const TRI: f32 = 0.0;

  pub fn new(osc_mode: OscillatorMode,osc_frequency: f32, amp: f32,is_on: bool,ratio: f32) -> SawNode {
    SawNode {
      common_oscillator: CommonOscillator::new(osc_mode,osc_frequency,amp,is_on),
      ratio: ratio
    }
  }
}

impl BaseOscillator for SawNode {
  
  fn get_common_oscillator(&mut self) -> &mut CommonOscillator {
    &mut (self.common_oscillator)
  }

  fn set_input_value_extended(&mut self, input: i32, value: f32){
    match input {
      SawNode::INPUT_RATIO => self.ratio = voltage_to_zero_to_one(value),
      _ => ()
    };
  }

  fn compute_extended(&mut self){
    let ratio = self.ratio;
    let nbr_samples_per_period = self.common_oscillator.sample_rate/self.common_oscillator.oscillator_frequency_hz;
    let middle = nbr_samples_per_period/2.0;
    let left_point = middle * ratio;
    let right_point = nbr_samples_per_period - left_point; 
    
    self.common_oscillator.value = if  self.common_oscillator.sample_clock < left_point {
      self.common_oscillator.sample_clock / left_point
    } else  if  self.common_oscillator.sample_clock < right_point {
      1.0 - (self.common_oscillator.sample_clock - left_point)/(middle - left_point)
    } else {
      - ( nbr_samples_per_period - self.common_oscillator.sample_clock) / left_point 
    } * self.common_oscillator.oscillator_amp;
  }
}