use std::rc::Weak;
use crate::synth::factory::audio_node_factory;
use crate::synth::factory::AUDIO_NODE_TYPE;

use crate::synth::audionode::AudioNode;
use std::rc::Rc;
use std::cell::RefCell;

//===============================================================
// Audio Link
//===============================================================

pub struct AudioLink<T> {
  remote_node: Weak<RefCell<Vertice<T>>>,
  remote_port: i32,
  local_port: i32
}

impl<T> AudioLink<T> {
  pub fn is_pointing_to_id(&self,id: &String) -> bool {
    let optional_rc_refcell_vertice: Option<Rc<RefCell<Vertice<T>>>> = self.remote_node.upgrade();
    match optional_rc_refcell_vertice {
      Some(rc_refcell_vertice) => {
        rc_refcell_vertice.borrow().id == *id
      },
      None => false
    }
  }

  pub fn matches(&self,local_port: i32, remote_id: &String, remote_port: i32 ) -> bool {
    local_port == self.local_port && remote_port == self.remote_port && self.is_pointing_to_id(remote_id)  
  }
}

#[cfg(test)]
mod testAudioLink {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(5, 5);
    }
}

//===============================================================
// Vertice
//===============================================================

pub struct Vertice<T> {
  id: String,
  payload: T,
  nbr_inputs: i32,
  nbr_outputs: i32,
  inputs:   Vec<AudioLink<T>>,
  outputs:   Vec<AudioLink<T>>
}

impl<T> Vertice<T> {
  pub fn remove_input_link(&mut self, local_port: i32, remote_id: &String, remote_port: i32){
    self.inputs.retain(|audio_link| !audio_link.matches(local_port,remote_id,remote_port));
  }
  pub fn remove_output_link(&mut self, local_port: i32, remote_id: &String, remote_port: i32){
    self.outputs.retain(|audio_link| !audio_link.matches(local_port,remote_id,remote_port));
  }
}


//===============================================================
// Graph
//===============================================================

pub struct Graph<T> {
  vertices : Vec<Rc<RefCell<Vertice<T>>>>
}

impl<T> Graph<T> {

  pub fn new() -> Graph<T> {
    Graph { 
      vertices: Vec::new()
    }
  }


  pub fn find_audio_node(&self,id: &String) -> Option<Rc<RefCell<Vertice<T>>>> {
    self.vertices.iter().find(move |v| (*v).borrow().id == *id ).cloned()
  }

  pub fn add_link(&mut self, 
                    output_node_id: &String, output_port: i32, 
                    input_node_id: &String,  input_port: i32) -> Result<(),String>{
    match self.find_audio_node(&output_node_id) {
      Some(output_vertice_ref) => {
        let output_weak = Rc::downgrade(&output_vertice_ref);
        match self.find_audio_node(&input_node_id) {
          Some(input_vertice_ref) => {
            input_vertice_ref.borrow_mut().inputs.push( AudioLink {
              remote_node: output_weak,
              remote_port: output_port,
              local_port: input_port
            });

            let input_weak = Rc::downgrade(&input_vertice_ref);
            output_vertice_ref.borrow_mut().outputs.push( AudioLink {
              remote_node: input_weak,
              remote_port: input_port,
              local_port: output_port
            });
            Ok(())
          }
          None => Err(String::from(format!("Input {} node does not exists.",input_node_id)))
        }
      }
      None => Err(String::from(format!("Output {} node does not exists.",output_node_id)))
    }

  }
}

//===============================================================
// DspGraph
//===============================================================

pub type DspGraph = Graph<Box<dyn AudioNode>>;
pub type DspVertice = Vertice<Box<dyn AudioNode>>;
pub type DspLink = AudioLink<Box<dyn AudioNode>>;

impl DspGraph {

  pub fn add_audio_node(&mut self, id:&String, typ: AUDIO_NODE_TYPE) {
      let audio_node = audio_node_factory(typ);
      self.vertices.push(Rc::new(RefCell::new(Vertice {
          id: id.clone(),
          payload: audio_node,
          nbr_inputs: 3,
          nbr_outputs: 2,
          inputs: Vec::new(),
          outputs: Vec::new()
      })));
    }

  #[inline(always)] 
  pub fn compute_vertice(&self, vertice: &mut DspVertice ) {
    let connected_inputs: Vec<&DspLink> = vertice.inputs.iter()
      .filter(|x| (*x).remote_node.upgrade().is_some())
      .collect();
    for input in connected_inputs {
      let output_value = input.remote_node.upgrade().unwrap().borrow().payload.get_output_value(input.remote_port);
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
