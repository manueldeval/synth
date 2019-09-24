use crate::synth::dsp::audionode::AudioNode;
use crate::synth::dsp::oscillators::simple::SinNode;
use crate::synth::dsp::oscillators::simple::SquareNode;
use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;

pub enum AudioNodeRegistry {
  SIN,
  SIN_LFO,
  SQUARE,
  SQUARE_LFO
}

impl AudioNodeRegistry {
  pub fn create_node(&self) -> Box<dyn AudioNode> {
    match self {
      AudioNodeRegistry::SIN        => Box::new(SinNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)),
      AudioNodeRegistry::SIN_LFO    => Box::new(SinNode::new(OscillatorMode::LFO, 1.0, 0.5, true)),
      AudioNodeRegistry::SQUARE     => Box::new(SquareNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)),
      AudioNodeRegistry::SQUARE_LFO => Box::new(SquareNode::new(OscillatorMode::LFO, 0.0, 0.5, true)),
    }
  }
}
