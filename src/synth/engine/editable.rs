use crate::synth::engine::synth::Synth;
use crate::graph::graph::Graph;
use crate::graph::graph::Vertice;
use crate::graph::graph::Link;
use crate::synth::dsp::audionode::AudioNode;
use crate::synth::dsp::registry::AudioNodeRegistry;
use crate::synth::dsp::various::identity::IdentityNode;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use crate::osc::osc::OSCReceiverFactory;

use std::fmt;
use crate::synth::engine::systemcommand::SystemCommandHandler;
//===============================================================
// DspGraph
//===============================================================

pub type DspGraph = Graph<Box<dyn AudioNode>>;
pub type DspVertice = Vertice<Box<dyn AudioNode>>;
pub type DspLink = Link<Box<dyn AudioNode>>;

impl DspGraph {

  pub fn add_audio_node(&mut self, id:&String, audio_node: Box<dyn AudioNode>) -> Result<(),String> {
    self.add_node(id,audio_node)
  }

  #[inline(always)] 
  pub fn compute_vertice(&self, vertice: &mut DspVertice ) {
    let connected_inputs: Vec<&DspLink> = vertice.inputs.iter()
      .filter(|x| (*x).remote_node.upgrade().is_some())
      .collect();
    for input in connected_inputs {
      // Unwrap is safety there, because we have filter previously the none case.
      let unwrapped = input.remote_node.upgrade().unwrap();

      // Uggly trick!
      // If the borrowing fails... it means that's a feedback on the current node!
      let output_value = match unwrapped.try_borrow() {
        Ok(borrowed)  => borrowed.payload.get_output_value(input.remote_port),
        Err(_)        => vertice.payload.get_output_value(input.remote_port)
      };

      vertice.payload.set_input_value(input.local_port,output_value);
    }
    vertice.payload.compute();
  }

  pub fn compute(&self) {
    for i in self.vertices.iter() {
      self.compute_vertice(&mut (*i).borrow_mut());
    }
  }
}



pub struct EditableSynth {
  pub graph: DspGraph,
  master_node: Weak<RefCell<DspVertice>>,
  sample_rate: i32,
  event_receiver_factory: OSCReceiverFactory
}

impl EditableSynth {
  pub const MASTER_ID: &'static str = "master"; 

  pub fn new(sample_rate: i32,osc_receiver_factory: OSCReceiverFactory) -> EditableSynth {
    let mut e = EditableSynth{ sample_rate: sample_rate, graph: DspGraph::new(), master_node: Weak::new(), event_receiver_factory: osc_receiver_factory };
    e.init().expect("Fatal: unable to create the master audio node.");
    e
  }

  fn init(&mut self) -> Result<(),String> {
    self.graph.reset();
    let mut master_audio_node =  AudioNodeRegistry::INDENTITY.create_node(self.sample_rate,&self.event_receiver_factory);
    master_audio_node.configure(self.sample_rate);
    self.graph.add_audio_node(&String::from(EditableSynth::MASTER_ID), master_audio_node)?;
    let master_node = Rc::downgrade(&self.graph.find_node(&String::from(EditableSynth::MASTER_ID)).unwrap());
    self.master_node = master_node;
    Ok(())
  }

  fn get_master_output(&self) -> f32 {
    match self.master_node.upgrade() {
      Some(e) => e.borrow().payload.get_output_value(IdentityNode::OUTPUT_VALUE),
      None => { 
        eprintln!("Strange... the master node is no more there...");
        0.0
      }
    }
  }

  pub fn compute(&mut self) -> f32 {
    self.graph.compute();
    self.get_master_output()
  }
  
}

impl Synth for EditableSynth {
  fn compute(&mut self) -> f32 {
    EditableSynth::compute(self)
  }
}

impl SystemCommandHandler for EditableSynth {

  fn add_audio_node(&mut self, id: &String, node_type: AudioNodeRegistry) -> Result<(),String> {
    let mut audio_node = node_type.create_node(self.sample_rate, &self.event_receiver_factory);
    audio_node.configure(self.sample_rate);
    self.graph.add_audio_node(id, audio_node)
  }

  fn add_link(&mut self, src_node: &String, src_port: i32, dst_node: &String, dst_port: i32) -> Result<(),String> {
    self.graph.add_link(src_node, src_port, dst_node, dst_port)
  }

  fn remove_link(&mut self, src_node: &String, src_port: i32, dst_node: &String, dst_port: i32) -> Result<(),String> {
    self.graph.remove_link(src_node, src_port, dst_node, dst_port)
  }

  fn remove_node(&mut self, id: &String) -> Result<(),String> {
    if id != EditableSynth::MASTER_ID {
      self.graph.remove_node(id)
    } else {
      Err(String::from(format!("The node {} cannot be removed.",EditableSynth::MASTER_ID)))
    }
  }

  fn rename_node(&mut self, old_id: &String, new_id: &String) -> Result<(),String> {
    if let Some(_) = self.graph.find_node(new_id)  {
      Err(String::from(format!("The node {} already exists.",new_id)))
    } else if let Some(node) = self.graph.find_node(old_id) {
      node.borrow_mut().id = new_id.clone();
      Ok(())
    } else {
      Err(String::from(format!("The node {} does not exists.",old_id)))
    }
  }

  fn reset(&mut self) -> Result<(),String> {
    self.init()
  }
}
