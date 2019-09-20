use super::converters::{voltage_to_lfo_frequency,voltage_to_frequency, voltage_to_zero_to_one,voltage_to_boolean,boolean_to_voltage};

pub enum OscillatorMode {
  LFO,
  AUDIO
}

pub struct CommonOscillator {
  pub oscillator_frequency_volt: f32,         // Current voltage
  pub oscillator_frequency_hz: f32,           // Computed value
  pub oscillator_amp: f32,
  pub sample_rate: f32,
  pub sample_clock: f32,
  pub value: f32,
  pub output_sync_phase: bool,
  pub input_sync_phase: bool,
  pub oscillator_mode: OscillatorMode,
  pub is_on: bool
}
use super::audionode::AudioNode;

impl CommonOscillator {
  pub const INPUT_FREQ: i32= 0;
  pub const INPUT_AMP: i32 = 1;
  pub const INPUT_TRIGGER: i32 = 2;
  pub const INPUT_TRIGGER_SYNC_PHASE: i32 = 3;

  pub const OUTPUT_OSC: i32 = 0;
  pub const OUTPUT_TRIGGER_SYNC_PHASE: i32 = 1;

  const INTERPOLATION_STEP: f32 = 0.005;

  pub fn new(osc_mode: OscillatorMode,osc_frequency: f32, amp: f32,is_on: bool) -> CommonOscillator {
    CommonOscillator { 
      oscillator_frequency_volt: osc_frequency, 
      oscillator_frequency_hz: 0.0,
      oscillator_amp: amp,
      sample_rate: 44_000_f32 ,
      sample_clock: 0_f32, 
      value: 0_f32,
      oscillator_mode: osc_mode,
      is_on: is_on,
      input_sync_phase: false,
      output_sync_phase: false
    }
  }
}


pub trait BaseOscillator {  
  fn get_common_oscillator(&self) -> &CommonOscillator;
  fn get_common_oscillator_mut(&mut self) -> &mut CommonOscillator;

  fn _set_input_freq(&mut self,value: f32){
    let mut common = self.get_common_oscillator_mut();

    let oscillator_target_frequency_volt = value;
    if (common.oscillator_frequency_volt - oscillator_target_frequency_volt).abs() < CommonOscillator::INTERPOLATION_STEP {
      common.oscillator_frequency_volt = oscillator_target_frequency_volt;
    } else if common.oscillator_frequency_volt > oscillator_target_frequency_volt {
      common.oscillator_frequency_volt -= CommonOscillator::INTERPOLATION_STEP;
    } else if common.oscillator_frequency_volt < oscillator_target_frequency_volt {
      common.oscillator_frequency_volt += CommonOscillator::INTERPOLATION_STEP;
    }
  }

  fn _set_input_amp(&mut self, value: f32){
    let mut common = self.get_common_oscillator_mut();
    common.oscillator_amp = voltage_to_zero_to_one(value);
  }

  fn _set_input_is_on(&mut self, value: f32){
    let mut common = self.get_common_oscillator_mut();
    common.is_on = voltage_to_boolean(value);
  }

  fn _set_input_sync_phase(&mut self, value: f32){
    let mut common = self.get_common_oscillator_mut();
    common.input_sync_phase = voltage_to_boolean(value);
  }

  fn set_input_value_extended(&mut self, _input: i32, _value: f32){
  }

  fn compute_extended(&mut self);
  
  fn get_output_value_extended(&self, _output: i32) -> f32 {
    0.0
  }
}

impl<T> AudioNode for T where T: BaseOscillator {  

  fn configure(&mut self,frequency: i32){
    self.get_common_oscillator_mut().sample_rate = frequency as f32;
  }
  
  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      CommonOscillator::INPUT_FREQ => self._set_input_freq(value),
      CommonOscillator::INPUT_AMP => self._set_input_amp(value),
      CommonOscillator::INPUT_TRIGGER => self._set_input_is_on(value),
      CommonOscillator::INPUT_TRIGGER_SYNC_PHASE => self._set_input_sync_phase(value),
      _ => self.set_input_value_extended(input,value)
    };
  }

  fn compute(&mut self) {
    let mut common = self.get_common_oscillator_mut();
    if common.is_on { 
      common.oscillator_frequency_hz = match common.oscillator_mode {
        OscillatorMode::LFO => voltage_to_lfo_frequency(common.oscillator_frequency_volt),
        OscillatorMode::AUDIO => voltage_to_frequency(common.oscillator_frequency_volt)
      };
      let old_sample_clock = common.sample_clock; 
      common.sample_clock = if common.input_sync_phase {
        0.0
      } else {
        (common.sample_clock + 1.0) % (common.sample_rate/common.oscillator_frequency_hz )
      };
      common.output_sync_phase = common.sample_clock < old_sample_clock;

      self.compute_extended();
    } else {
      common.value = 0.0;
    }
  }

  fn get_output_value(&self, output: i32) -> f32 { 
    let mut common = self.get_common_oscillator();
    match output {
      CommonOscillator::OUTPUT_OSC => common.value,
      CommonOscillator::OUTPUT_TRIGGER_SYNC_PHASE => boolean_to_voltage(common.output_sync_phase),
      _ =>  self.get_output_value_extended(output)
    }
  }
}
