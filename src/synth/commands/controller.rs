use crate::graph::graph::*;
use crate::synth::commands::systemcommand::*;
use crate::synth::dsp::registry::*;
use crate::synth::commands::config::*;
use crossbeam::crossbeam_channel::*;
use std::thread::JoinHandle;
use std::thread;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

struct ControllerNode {
}
impl ControllerNode {
  pub fn new() -> ControllerNode {
    ControllerNode {}
  }  
}
type CtrlGraph = Graph<ControllerNode>;
type CtrlVertice = Vertice<ControllerNode>;
type CtrlLink = Link<ControllerNode>;


struct OrderVisitor{
  ordered_ids: Vec<String>
}
impl OrderVisitor {
  fn new() -> OrderVisitor {
    OrderVisitor{ ordered_ids: Vec::new() }
  }
}
impl Visitor<ControllerNode,Vec<String>> for OrderVisitor {
  fn visit(&mut self, vertice: &mut CtrlVertice){
    self.ordered_ids.push(vertice.id.clone());
    // Push front.
    self.ordered_ids.rotate_right(1);
  }
  fn result(&self) -> Vec<String>{
    self.ordered_ids.clone()
  }
}

pub struct CommandController {
  graph: CtrlGraph,
  master_node: Weak<RefCell<CtrlVertice>>,
  sender: Sender<SystemCommand>
}

impl CommandController {
  pub const MASTER_ID: &'static str = "master"; 

  fn new(sender: Sender<SystemCommand>) -> CommandController {
    let graph = CtrlGraph::new();
    let mut cc = CommandController { sender, graph , master_node: Weak::new()};
    let _ = cc.init();
    cc
  }


  fn init(&mut self) -> Result<(),String> {
    self.graph.reset();
    let master_audio_node =  ControllerNode::new();
    self.graph.add_node(&String::from(CommandController::MASTER_ID), master_audio_node)?;
    let master_node = Rc::downgrade(&self.graph.find_node(&String::from(CommandController::MASTER_ID)).unwrap());
    self.master_node = master_node;
    Ok(())
  }  

  pub fn compute_new_graphe_eorder(&mut self) -> Result<(),String> {
    let ordered_vect = self.graph.idfs(&String::from(CommandController::MASTER_ID), &mut OrderVisitor::new())?;
    self.graph.reorder(&ordered_vect)?;
    match self.sender.send(SystemCommand::Redorder { order: ordered_vect }) {
      Ok(()) => Ok(()),
      Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
    }?;
    Ok(())
  }
}

impl SystemCommandHandler for CommandController {

  fn add_audio_node(&mut self, id: &String, node_type: AudioNodeRegistry) -> Result<(),String> {
    self.graph.add_node(id, ControllerNode::new())?;
    match self.sender.send(SystemCommand::Create { id: id.clone(), node_type: node_type.clone() }) {
      Ok(()) => Ok(()),
      Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
    }?;
    self.compute_new_graphe_eorder()
  }

  fn add_link(&mut self, src_node: &String, src_port: i32, dst_node: &String, dst_port: i32) -> Result<(),String>  {
    self.graph.add_link(src_node, src_port, dst_node, dst_port)?;
    match self.sender.send(SystemCommand::Link { src_node: src_node.clone(), src_port, dst_node: dst_node.clone(), dst_port }) {
      Ok(()) => Ok(()),
      Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
    }?;
    self.compute_new_graphe_eorder()
  }

  fn remove_link(&mut self, src_node: &String, src_port: i32, dst_node: &String, dst_port: i32) -> Result<(),String>  {
    self.graph.remove_link(src_node, src_port, dst_node, dst_port)?;
    match self.sender.send(SystemCommand::Unlink { src_node: src_node.clone(), src_port, dst_node: dst_node.clone(), dst_port }) {
      Ok(()) => Ok(()),
      Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
    }?;
    self.compute_new_graphe_eorder()
  }

  fn remove_node(&mut self, id: &String) -> Result<(),String> {
    if id != CommandController::MASTER_ID {
      self.graph.remove_node(id)?;
      match self.sender.send(SystemCommand::Remove { id:id.clone() }) {
        Ok(()) => Ok(()),
        Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
      }?;
      self.compute_new_graphe_eorder()
    } else {
      Err(String::from(format!("The node {} cannot be removed.",CommandController::MASTER_ID)))
    }  
  }

  fn rename_node(&mut self, old_id: &String, new_id: &String) -> Result<(),String> {
    if old_id == CommandController::MASTER_ID{
      Err(String::from(format!("Cannot rename master node.")))
    } else if let Some(_) = self.graph.find_node(new_id)  {
      Err(String::from(format!("The node {} already exists.",new_id)))
    } else if let Some(node) = self.graph.find_node(old_id) {
      node.borrow_mut().id = new_id.clone();
      match self.sender.send(SystemCommand::Rename { old_id:old_id.clone(), new_id: new_id.clone() }) {
        Ok(()) => Ok(()),
        Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
      }
    } else {
      Err(String::from(format!("The node {} does not exists.",old_id)))
    }
  }

  fn reset(&mut self) -> Result<(),String> {
    self.init()?;
    match self.sender.send(SystemCommand::Reset) {
      Ok(()) => Ok(()),
      Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
    }
  }

  fn change_config(&mut self,id: &String, key: &String, val: &ConfigVal) -> Result<(),String> {
    if let Some(_) = self.graph.find_node(id) {
      match self.sender.send(SystemCommand::ChangeConfig { id: id.clone(), key: key.clone() , val: val.clone() }) {
        Ok(()) => Ok(()),
        Err(e) => Err(String::from(format!("Unable to send the message: {}",e)))
      }
    } else {
      Err(String::from(format!("The node {} does not exists.",id)))
    }
  }

  fn reorder(&mut self, order: &Vec<String>) -> Result<(),String> {
    Err(String::from(format!("Command reorder not supported.")))
  }

}

pub struct CommandControllerThread {
  receiver: Receiver<SystemCommand>,
  sender: Sender<SystemCommand>
}

impl CommandControllerThread {
  pub fn new(receiver: Receiver<SystemCommand>, sender: Sender<SystemCommand>) -> CommandControllerThread {
    CommandControllerThread{ receiver, sender }
  }

  pub fn start(&self) -> JoinHandle<()> {
    let rec = self.receiver.clone();
    let snd = self.sender.clone();
    thread::spawn(move || {
      let mut command_controller : CommandController = CommandController::new(snd);
      loop {
        if let Ok(command) = rec.recv()  {
          match command_controller.receive_command(&command){
            Err(e) => eprintln!("{}",e),
            _ => ()
          };
        };
      }
    })
  }

}
