use std::rc::Weak;
use crate::synth::factory::audio_node_factory;
use crate::synth::factory::AUDIO_NODE_TYPE;

use crate::synth::audionode::AudioNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct AudioLink {
  remote_node: Weak<RefCell<Vertice>>,
  remote_port: i32,
  local_port: i32
}

pub struct Vertice {
  id: String,
  payload: Box<dyn AudioNode>,
  nbr_inputs: i32,
  nbr_outputs: i32,
  inputs:   Vec<AudioLink>,
  outputs:   Vec<AudioLink>
}

/**
 * Graph
 */
pub struct Graph {
  vertices : Vec<Rc<RefCell<Vertice>>>
}

impl Graph {

  pub fn new() -> Graph {
    Graph { 
      vertices: Vec::new()
    }
  }

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

  pub fn find_audio_node(&self,id: &String) -> Option<Rc<RefCell<Vertice>>> {
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


  #[inline(always)] 
  pub fn compute_vertice(&self, vertice: &mut Vertice ) {
    let connected_inputs: Vec<&AudioLink> = vertice.inputs.iter()
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


// /**
//  * Vertice
//  */
// #[derive(Clone, Debug)]
// pub struct Vertice<T> where T: Eq + Clone {
//   id: Box<T>
// }

// impl<T> Vertice<T> where T: Eq + Clone {
//   pub fn new(t: T) -> Vertice<T> {
//     Vertice {
//       id: Box::new(t)
//     }
//   }
// }

// impl<T> PartialEq for Vertice<T> where T: Eq + Clone {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

// impl<T> Eq for Vertice<T> where T: Eq + Clone {
// }

// /**
//  * Edge
//  */
// #[derive(Clone, Debug)]
// pub struct Edge<T> where T: Eq + Clone {
//   src: Box<T>,
//   dst: Box<T>
// } 

// impl<T> Edge<T> where T: Eq + Clone {
//   pub fn new(src: T,dst: T) -> Edge<T> {
//     Edge {
//       src: Box::new(src),
//       dst: Box::new(dst)
//     }
//   }
// }

// impl<T> PartialEq for Edge<T> where T: Eq + Clone {
//     fn eq(&self, other: &Self) -> bool {
//         self.src == other.src && self.dst == other.dst
//     }
// }

// impl<T> Eq for Edge<T> where T: Eq + Clone {
// }

// /**
//  * Graph
//  */
// pub struct Graph<T>  where T: Eq + Clone {

//   vertices: Vec<Vertice<T>>,
//   edges: Vec<Edge<T>>

// }

// impl<T> Graph<T> where T: Eq + Clone {

//   pub fn new() -> Graph<T> {
//     Graph { 
//       vertices: Vec::new(),
//       edges: Vec::new()
//     }
//   }

//   //=== Vertices
//   fn index_of_vertice(&self,t: &T) -> Option<usize> {
//     self.vertices.iter().position(|x| *(x.id) == *t )
//   }

//   pub fn exist_vertice(&self, t: &T) -> bool {
//     self.index_of_vertice(t).is_some()
//   }

//   pub fn add_vertice(&mut self, t: &T) -> Result<(),String> {
//     if self.exist_vertice(t) {
//       Err(String::from("Vertice already exists."))
//     } else {
//       self.vertices.push(Vertice::new(t.clone()));
//       Ok(())
//     }
//   }

//   fn index_of_edge(&self,src: &T, dst: &T) -> Option<usize> {
//     let target_edge = Edge::new(src.clone(),dst.clone());
//     self.edges.iter().position(move |edge| (*edge) == target_edge )
//   }

//   fn get_edges_starting_by_vertice(&self,src: &T) -> Vec<Edge<T>> {
//     self.edges.iter()
//       .filter(|edge| *(edge.src) == *src)
//       .cloned()
//       .collect::<Vec<Edge<T>>>()
//   }

//   fn get_edges_ending_by_vertice(&self,dst: &T) -> Vec<Edge<T>> {
//     self.edges.iter()
//       .filter(|edge| *(edge.dst) == *dst)
//       .cloned()
//       .collect::<Vec<Edge<T>>>()
//   }

//   //=== Edges
//   pub fn exist_edge(&self, src: &T, dst: &T) -> bool {
//     self.index_of_edge(src, dst).is_some()
//   }

//   pub fn add_edge(&mut self, src: &T, dst: &T) -> Result<(),String> {
//     if !self.exist_vertice(src) {
//       Err(String::from("Src vertice does not exists."))
//     } else if !self.exist_vertice(dst){
//       Err(String::from("Dst vertice does not exists."))
//     } else if self.exist_edge(src, dst) {
//       Err(String::from("Edge already exists."))
//     } else {
//       self.edges.push(Edge::new(src.clone(), dst.clone()));
//       Ok(())
//     }
//   }

//   pub fn remove_edge(&mut self, src: &T, dst: &T) -> Result<(),String> {
//     let idx_opt = self.index_of_edge(src, dst);
//     match idx_opt {
//       Some(idx) => { 
//         self.edges.remove(idx);
//         Ok(())
//       }
//       None => Err(String::from("Edge dos not exists."))
//     }
//   }
// }
