use serde::{Serialize, Deserialize};
use crate::synth::utils::converters::*;
use crate::synth::utils::note::*;

pub trait ToVolt {
  fn volts(&self) -> f32;
}

/*
========================================
Audion OSC Frequency 
=========================================
*/
#[derive(Serialize, Deserialize,Clone)]
pub enum OscFreq {
  Hz(f32),
  Val(f32),
  Note(Note)
}
impl ToVolt for OscFreq {
  fn volts(&self) -> f32 {
    match &self {
      OscFreq::Hz(v) => voltage_to_frequency(*v),
      OscFreq::Val(v) => *v,
      OscFreq::Note(note) => (*note).to_voltage()
    }
  }
}
pub type OptOscFreq = Option<OscFreq>;

/*
========================================
Audion OSC Frequency 
=========================================
*/
#[derive(Serialize, Deserialize,Clone)]
pub enum Amp {
  Lin(f32),
  Val(f32)
}

impl ToVolt for Amp {
  fn volts(&self) -> f32 {
    match &self {
      Amp::Lin(v) => *v,
      Amp::Val(v) => *v
    }
  }
}
pub type OptAmp = Option<Amp>;
