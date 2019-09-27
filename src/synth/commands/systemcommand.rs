use crate::synth::dsp::registry::AudioNodeRegistry;
use std::fmt;
use serde::{Serialize, Deserialize};

pub trait SystemCommandHandler {
  fn add_audio_node(&mut self, id: &String, node_type: AudioNodeRegistry) -> Result<(),String> ;
  fn add_link(&mut self, src_node: &String, src_port: i32, dst_node: &String, dst_port: i32) -> Result<(),String> ;
  fn remove_link(&mut self, src_node: &String, src_port: i32, dst_node: &String, dst_port: i32) -> Result<(),String> ;
  fn remove_node(&mut self, id: &String) -> Result<(),String>;
  fn rename_node(&mut self, old_id: &String, new_id: &String) -> Result<(),String>;
  fn reset(&mut self) -> Result<(),String>;
  fn receive_command(&mut self, command: &SystemCommand) -> Result<(),String> {
    match command {
      SystemCommand::Create { id, node_type } => self.add_audio_node(id, node_type.clone()),
      SystemCommand::Link {src_node, src_port, dst_node, dst_port} =>  self.add_link(src_node, *src_port, dst_node, *dst_port),
      SystemCommand::Unlink {src_node, src_port, dst_node, dst_port} => self.remove_link(src_node, *src_port, dst_node, *dst_port),
      SystemCommand::Remove { id } => self.remove_node(id),
      SystemCommand::Reset => self.reset(),
      SystemCommand::Rename { old_id, new_id } => self.rename_node(old_id,new_id)
    }
  }
}

#[derive(Serialize, Deserialize,Clone)]
pub enum SystemCommand {
  Create {id: String, node_type: AudioNodeRegistry },
  Link  {src_node: String, src_port: i32, dst_node: String, dst_port: i32},
  Unlink {src_node: String, src_port: i32, dst_node: String, dst_port: i32},
  Remove { id: String },
  Rename { old_id: String, new_id: String },
  Reset
  // ,
  // ChangeConfig { id: String, key: String, val: ConfigVal }
}

impl fmt::Display for SystemCommand {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        SystemCommand::Create { id, node_type }                        => write!(f,"Create (id: {}, node_type: {} )",id,node_type),
        SystemCommand::Link {src_node, src_port, dst_node, dst_port}   => write!(f,"Link   (src_node: {}, src_port: {}, dst_node: {}, dst_port: {})",src_node,src_port,dst_node,dst_port),
        SystemCommand::Unlink {src_node, src_port, dst_node, dst_port} => write!(f,"Unlink (src_node: {}, src_port: {}, dst_node: {}, dst_port: {})",src_node,src_port,dst_node,dst_port),
        SystemCommand::Remove { id }                                   => write!(f,"Remove (id: {})",id),
        SystemCommand::Reset                                           => write!(f,"Reset"),
        SystemCommand::Rename {old_id , new_id }                       => write!(f,"Rename (old_id: {}, new_id:{})",old_id,new_id)
      }
    }
}
/*
SystemCommand     => mutation de synt (10/s)
RealtimeCommands  => mutation de synt (1000/s)
Sound  => mutation de synt (44_000/s)
*/