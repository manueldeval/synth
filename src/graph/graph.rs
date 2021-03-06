use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::fmt;

//===============================================================
// Link
//===============================================================

pub struct Link<T> {
  pub remote_node: Weak<RefCell<Vertice<T>>>,
  pub remote_port: i32,
  pub local_port: i32
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
  pub id: String,
  pub payload: T,
  pub inputs:   Vec<Link<T>>,
  pub outputs:   Vec<Link<T>>
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

  pub fn remove_input_links_from(&mut self, remote_id: &String) -> Result<(),String> {
    let before = self.inputs.len();
    self.inputs.retain(|audio_link| !audio_link.target_is(remote_id));
    let after = self.inputs.len();
    if before != after {
      Ok(())
    } else {
      Err(String::from(format!("Input link on Node {} to node {} does not exists.",self.id,remote_id)))
    }
  }

  pub fn remove_output_links_from(&mut self, remote_id: &String) -> Result<(),String> {
    let before = self.outputs.len();
    self.outputs.retain(|audio_link| !audio_link.target_is(remote_id));
    let after = self.outputs.len();
    if before != after {
      Ok(())
    } else {
      Err(String::from(format!("Output link on Node {} to node {} does not exists.",self.id,remote_id)))
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
          writeln!(f, "    Input[{}] <= Output[{}] of vertice: '{}'", input.local_port,input.remote_port,input_node_name)?;
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


use std::collections::HashSet;
use std::ops::DerefMut;
use std::cmp::Ordering;
//===============================================================
// Graph
//===============================================================
pub trait Visitor<T,U> {
  fn visit(&mut self, vertice: &mut Vertice<T>);
  fn result(&self) -> U;
}

pub struct Graph<T> {
  pub vertices : Vec<Rc<RefCell<Vertice<T>>>>
}

impl<T> Graph<T> {

  pub fn new() -> Graph<T> {
    Graph { 
      vertices: Vec::new()
    }
  }
  // TRAVERSAL
  pub fn idfs<U>(&self,id: &String,visitor: &mut dyn Visitor<T,U>) -> Result<U, String>{
    // If the first node does not exits, skip.

    let mut visited : HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = Vec::new();
    stack.push(id.clone());
    loop {
      match stack.pop() {
        // While stack is not empty...
        Some(current_id) => {
          if !visited.contains(&current_id) {
            let current_node = self
              .find_node(&current_id)
              .ok_or(String::from(format!("Node {} does not exists",current_id)))?;

            // Callbak?
            visitor.visit(current_node.borrow_mut().deref_mut());

            // Mark as visited
            visited.insert(current_id.clone());

            // Push child nodes
            let parents: Vec<String>  = current_node.borrow().inputs.iter()
              .flat_map(|link| link.remote_node.upgrade())
              .map(|remote_node| remote_node.borrow().id.clone())
              .filter(|id| id != &current_id)
              .collect();
            for parent_id in parents.iter() {
              stack.push(parent_id.clone());
            }
          };
        },
        None => { break; }
      }
    }
    Ok(visitor.result())
  }

  pub fn reorder(&mut self, new_ids_order: &Vec<String>) -> Result<(),String> {
    self.vertices.sort_by(|v1, v2| {
      let id1 = v1.borrow().id.clone();
      let id2 = v2.borrow().id.clone();
      let pos1 = new_ids_order.iter().position(|r| *r == id1);
      let pos2 = new_ids_order.iter().position(|r| *r == id2);
      match (pos1,pos2){
        (Some(a),Some(b)) => a.cmp(&b),
        (Some(_),None) => Ordering::Greater,
        (None,Some(_)) => Ordering::Less,
        (None,None) => Ordering::Equal
      }
    });
    Ok(())
  }

  pub fn add_node(&mut self,id: &String,t: T) -> Result<(),String> {
    match self.find_node(id) {
      None => {
        self.vertices.push(Rc::new(RefCell::new(Vertice::new(id,t))));
        Ok(())
      },
      Some(_) => Err(String::from(format!("Cannot add the node with id {}, id already exists.",id)))
    } 
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

  pub fn reset(&mut self) {
    self.vertices.truncate(0);
  }

  pub fn remove_node(&mut self,node_id: &String) -> Result<(),String> {
    // 1) Find all Vertices pointing on self.
    let vertice_to_remove_ref = self.find_node(&node_id).ok_or(String::from(format!("Node {} node does not exists.",node_id)))?;
    let nodes_with_output_to_clean: Vec<String> = vertice_to_remove_ref.borrow().inputs.iter()
              .flat_map(|link| link.remote_node.upgrade())
              .map(|remote_node| remote_node.borrow().id.clone())
              .filter(|id| id != node_id)
              .collect();
    for node in nodes_with_output_to_clean.iter() {
        let target_node = self.find_node(&node).ok_or(String::from(format!("Node {} node does not exists.",node)))?;
        let _ = target_node.borrow_mut().remove_output_links_from(node_id);
    }

    let nodes_with_input_to_clean: Vec<String> = vertice_to_remove_ref.borrow().outputs.iter()
              .flat_map(|link| link.remote_node.upgrade())
              .map(|remote_node| remote_node.borrow().id.clone())
              .filter(|id| id != node_id)
              .collect();
    for node in nodes_with_input_to_clean.iter() {
        let target_node = self.find_node(&node).ok_or(String::from(format!("Node {} node does not exists.",node)))?;
        let _ = target_node.borrow_mut().remove_input_links_from(node_id);
    }

    // 3) Remove node
    self.vertices.retain(|v| !(v.borrow().id == *node_id)) ;
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
