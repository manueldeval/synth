use crate::graph::graph::DspGraph;
use crate::synth::engine::synth::Synth;

pub struct EditableSynth {
  graph: DspGraph
}

impl Synth for EditableSynth {
  fn compute(&mut self) -> f32 {
    1.0
  }
}
