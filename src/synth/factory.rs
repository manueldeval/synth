use crate::synth::audionode::AudioNode;
use crate::synth::oscillators::SinNode;
use crate::synth::oscillators::SquareNode;
use crate::synth::baseoscillator::OscillatorMode;

pub enum AUDIO_NODE_TYPE {
  SIN,
  SIN_LFO,
  SQUARE,
  SQUARE_LFO
}

pub fn audio_node_factory(t: AUDIO_NODE_TYPE) -> Box<dyn AudioNode> {
  match t {
    AUDIO_NODE_TYPE::SIN        => Box::new(SinNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)),
    AUDIO_NODE_TYPE::SIN_LFO    => Box::new(SinNode::new(OscillatorMode::LFO, 1.0, 0.5, true)),
    AUDIO_NODE_TYPE::SQUARE     => Box::new(SquareNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)),
    AUDIO_NODE_TYPE::SQUARE_LFO => Box::new(SquareNode::new(OscillatorMode::LFO, 0.0, 0.5, true)),
  }
}