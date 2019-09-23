use std::rc::Weak;
use crate::synth::factory::audio_node_factory;
use crate::synth::factory::AUDIO_NODE_TYPE;

use crate::synth::audionode::AudioNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

//===============================================================
// Link
//===============================================================

pub struct Link<T> {
  remote_node: Weak<RefCell<Vertice<T>>>,
  remote_port: i32,
  local_port: i32
}

impl<T> Link<T> {
  pub fn new(local_port: i32, weak_remote_node: Weak<RefCell<Vertice<T>>>, remote_port: i32) -> Link<T> {
    Link {
      remote_node: weak_remote_node,
      remote_port: remote_port,
      local_port: local_port
    }
  }

  pub fn target_is(&self,id: &String) -> bool {
    let optional_rc_refcell_vertice: Option<Rc<RefCell<Vertice<T>>>> = self.remote_node.upgrade();
    match optional_rc_refcell_vertice {
      Some(rc_refcell_vertice)  => rc_refcell_vertice.borrow().id == *id,
      None                      => false
    }
  }

  pub fn matches(&self,local_port: i32, remote_id: &String, remote_port: i32 ) -> bool {
    local_port == self.local_port && remote_port == self.remote_port && self.target_is(remote_id)  
  }
}

//===============================================================
// Vertice
//===============================================================

pub struct Vertice<T> {
  id: String,
  payload: T,
  inputs:   Vec<Link<T>>,
  outputs:   Vec<Link<T>>
}

impl<T> Vertice<T> {
  pub fn new(id: &String, t: T) -> Vertice<T> {
    Vertice {
      id: id.clone(),
      payload: t,
      inputs: Vec::new(),
      outputs: Vec::new()
    }
  }

  pub fn remove_input_link(&mut self, local_port: i32, remote_id: &String, remote_port: i32) -> Result<(),String> {
    let before = self.inputs.len();
    self.inputs.retain(|audio_link| !audio_link.matches(local_port,remote_id,remote_port));
    let after = self.inputs.len();
    if before != after {
      Ok(())
    } else {
      Err(String::from(format!("Input link on Node {} [{}] to node {} [{}] does not exists.",self.id,local_port,remote_id,remote_port)))
    }
  }
  pub fn remove_output_link(&mut self, local_port: i32, remote_id: &String, remote_port: i32) -> Result<(),String> {
    let before = self.outputs.len();
    self.outputs.retain(|audio_link| !audio_link.matches(local_port,remote_id,remote_port));
    let after = self.outputs.len();
    if before != after {
      Ok(())
    } else {
      Err(String::from(format!("Output link on Node {} [{}] to node {} [{}] does not exists.",self.id,local_port,remote_id,remote_port)))
    }
  }

  pub fn add_input_link(&mut self,local_port: i32, weak_remote_node: Weak<RefCell<Vertice<T>>>, remote_port: i32){
    self.inputs.push(Link::new(local_port,weak_remote_node,remote_port));
  }
  pub fn add_output_link(&mut self,local_port: i32, weak_remote_node: Weak<RefCell<Vertice<T>>>, remote_port: i32){
    self.outputs.push(Link::new(local_port,weak_remote_node,remote_port));
  }
}

impl<T> fmt::Display for Vertice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "==== Vertice ID: {} ====", self.id)?;
        writeln!(f, "---- Outputs: ----")?;
        for output in self.outputs.iter() {
          let output_node_name = output.remote_node.upgrade().map_or_else(|| String::from("[!Dropped!]"),|n| n.borrow().id.clone());
          writeln!(f, "    Output[{}] => Input[{}] of vertice: '{}'", output.local_port,output.remote_port,output_node_name)?;
        }
        writeln!(f, "---- Inputs: ----")?;
        for input in self.inputs.iter() {
          let input_node_name = input.remote_node.upgrade().map_or_else(|| String::from("[!Dropped!]"),|n| n.borrow().id.clone());
          writeln!(f, "    Input[{}] => Output[{}] of vertice: '{}'", input.local_port,input.remote_port,input_node_name)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod test_vertices {
    use super::*;

    #[test]
    fn test_create_vertice() {
      let v : Vertice<i32> = Vertice::new(&String::from("toto"),42);
      assert_eq!(v.payload, 42);
      assert_eq!(v.inputs.len(),0);
      assert_eq!(v.outputs.len(),0);
    }

    #[test]
    fn test_link() {
      let node1 = &String::from("1");
      let node2 = &String::from("2");

      let v1 : Rc<RefCell<Vertice<i32>>> = Rc::new(RefCell::new(Vertice::new(node1,1)));
      let v2 : Rc<RefCell<Vertice<i32>>> = Rc::new(RefCell::new(Vertice::new(node2,2)));

      v1.borrow_mut().add_input_link(0,Rc::downgrade(&v2), 1);
      v2.borrow_mut().add_output_link(1,Rc::downgrade(&v1), 0);
      println!("{}",v1.clone().borrow());
      println!("{}",v2.clone().borrow());
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

  pub fn add_node(&mut self,id: &String,t: T){
    self.vertices.push(Rc::new(RefCell::new(Vertice::new(id,t)))); 
  }


  pub fn find_node(&self,id: &String) -> Option<Rc<RefCell<Vertice<T>>>> {
    self.vertices.iter().find(move |v| (*v).borrow().id == *id ).cloned()
  }

  pub fn add_link(&mut self, 
                    output_node_id: &String, output_port: i32, 
                    input_node_id: &String,  input_port: i32) -> Result<(),String> {

    let output_vertice_ref = self.find_node(&output_node_id).ok_or(String::from(format!("Output {} node does not exists.",output_node_id)))?;
    let input_vertice_ref = self.find_node(&input_node_id).ok_or(String::from(format!("Input {} node does not exists.",input_node_id)))?;

    let output_weak = Rc::downgrade(&output_vertice_ref);
    input_vertice_ref.borrow_mut().add_input_link(input_port,output_weak,output_port);

    let input_weak = Rc::downgrade(&input_vertice_ref);
    output_vertice_ref.borrow_mut().add_output_link(output_port,input_weak,input_port);
    Ok(())
  }

  pub fn remove_link(&mut self, 
                    output_node_id: &String, output_port: i32, 
                    input_node_id: &String,  input_port: i32) -> Result<(),String> {


    let output_vertice_ref = self.find_node(&output_node_id).ok_or(String::from(format!("Output {} node does not exists.",output_node_id)))?;
    let input_vertice_ref = self.find_node(&input_node_id).ok_or(String::from(format!("Input {} node does not exists.",input_node_id)))?;

    input_vertice_ref.borrow_mut().remove_input_link(input_port, output_node_id, output_port)?;
    output_vertice_ref.borrow_mut().remove_output_link(output_port, input_node_id, input_port)?;

    Ok(())
  }

  pub fn remove_node(&mut self,node_id: &String) -> Result<(),String> {
    // 1) Remove all links (in/out)

    // 2) Remove node
    Ok(())
  }

}

impl<T> fmt::Display for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"########### GRAPH ###########")?;
        for vertice in self.vertices.iter() {
          vertice.borrow().fmt(f)?;
        }
        Ok(())
    }
}

//===============================================================
// DspGraph
//===============================================================

pub type DspGraph = Graph<Box<dyn AudioNode>>;
pub type DspVertice = Vertice<Box<dyn AudioNode>>;
pub type DspLink = Link<Box<dyn AudioNode>>;

impl DspGraph {

  pub fn add_audio_node(&mut self, id:&String, typ: AUDIO_NODE_TYPE) {
      let audio_node = audio_node_factory(typ);
      self.add_node(id,audio_node);
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
