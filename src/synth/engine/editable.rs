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

//===============================================================
// DspGraph
//===============================================================

pub type DspGraph = Graph<Box<dyn AudioNode>>;
pub type DspVertice = Vertice<Box<dyn AudioNode>>;
pub type DspLink = Link<Box<dyn AudioNode>>;

impl DspGraph {

  pub fn add_audio_node(&mut self, id:&String, audio_node: Box<dyn AudioNode>) {
      self.add_node(id,audio_node);
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
    let mut graph = DspGraph::new(); 
    graph.add_audio_node(&String::from(EditableSynth::MASTER_ID), AudioNodeRegistry::INDENTITY.create_node(sample_rate, &osc_receiver_factory));
    let master_node =Rc::downgrade(&graph.find_node(&String::from(EditableSynth::MASTER_ID)).unwrap());
    EditableSynth{ sample_rate: sample_rate, graph: graph, master_node: master_node, event_receiver_factory: osc_receiver_factory }
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

  pub fn receive_command(&mut self, command: &EditableSynthCommand) -> Result<(),String> {
    match command {
      EditableSynthCommand::Create { id, node_type } if id != EditableSynth::MASTER_ID => {
        self.graph.add_audio_node(id, (*node_type).create_node(self.sample_rate, &self.event_receiver_factory) );
        Ok(())
      },
      EditableSynthCommand::Link {src_node, src_port, dst_node, dst_port} => {
        self.graph.add_link(src_node, *src_port, dst_node, *dst_port)
      },
      EditableSynthCommand::Unlink {src_node, src_port, dst_node, dst_port} => {
        self.graph.remove_link(src_node, *src_port, dst_node, *dst_port)
      },
      EditableSynthCommand::Remove { id }  if id != EditableSynth::MASTER_ID => {
        self.graph.remove_node(id)
      },
      _ => Err(String::from(format!("Unable to execute command: {}",command)))
    }
  }
  
}

impl Synth for EditableSynth {
  fn compute(&mut self) -> f32 {
    EditableSynth::compute(self)
  }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone)]
pub enum EditableSynthCommand {
  Create {id: String, node_type: AudioNodeRegistry },
  Link  {src_node: String, src_port: i32, dst_node: String, dst_port: i32},
  Unlink {src_node: String, src_port: i32, dst_node: String, dst_port: i32},
  Remove { id: String }
}

impl fmt::Display for EditableSynthCommand {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        EditableSynthCommand::Create { id, node_type }                        => write!(f,"Create (id: {}, node_type: {} )",id,node_type),
        EditableSynthCommand::Link {src_node, src_port, dst_node, dst_port}   => write!(f,"Link   (src_node: {}, src_port: {}, dst_node: {}, dst_port: {})",src_node,src_port,dst_node,dst_port),
        EditableSynthCommand::Unlink {src_node, src_port, dst_node, dst_port} => write!(f,"Unlink (src_node: {}, src_port: {}, dst_node: {}, dst_port: {})",src_node,src_port,dst_node,dst_port),
        EditableSynthCommand::Remove { id }                                   => write!(f,"Remove (id: {})",id),
      }
    }
}