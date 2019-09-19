use crate::synth::audionode::AudioNode;

use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use piston_window::*;


/*
=========================================
Viz
=========================================
*/
#[derive(Clone)]
pub struct ScopeNodeData {
  pub value: f32,
  pub buffer: [f32; ScopeNode::BUFFER_SIZE],
  pub actual_size: usize,
  pub current_index: usize,
}


pub struct ScopeNode {
  data: ScopeNodeData,
  tx: Sender<ScopeNodeData>
}

impl ScopeNode {
  pub const BUFFER_SIZE: usize = 1_000; 
  pub const INPUT_SIGNAL: i32 = 0;

  pub fn new() -> ScopeNode {
    let tx = ScopeNode::launch_scope();

    let scope = ScopeNode { 
        tx: tx,
        data: ScopeNodeData {
          value: 0.0, 
          buffer:[0.0; ScopeNode::BUFFER_SIZE],
          actual_size: ScopeNode::BUFFER_SIZE,
          current_index: 0,
      }
    };
    scope
  }


    // some work here

  pub fn launch_scope() -> Sender<ScopeNodeData> {
    let  (tx, rx): (Sender<ScopeNodeData>, Receiver<ScopeNodeData>) = channel();
    thread::spawn(move || {

        let mut window: PistonWindow = WindowSettings::new("Scope", [640, 480])
                    .resizable(false)
                    .exit_on_esc(false)
                    .build()
                    .unwrap();
        let mut buffer : [f32;ScopeNode::BUFFER_SIZE] = [0.0;ScopeNode::BUFFER_SIZE];
        while let Some(event) = window.next() {

          let rec = rx.try_recv();

          buffer = match rec { Ok(data) => data.buffer, _ => buffer };
          // buffer = match rec { Ok(data) => data.buffer, _ => buffer };
          let mut zero_cross = 0;
          for i in 1..320 {
            if buffer[i-1]<0.0 && buffer[i]>0.0 {
              zero_cross = i;
            }
          }
          
          window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            for i in 0..640 {
              let value = (400.0*(buffer[i+zero_cross]+1.0)/2.0) as f64;
              rectangle([1.0, 0.0, 0.0, 1.0], 
                        [i as f64, value+40.0,1.0 , 1.0 ],
                        context.transform,
                        graphics);
            }
          });
        }   
    });

    tx
  }

}

impl AudioNode for ScopeNode {

  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      ScopeNode::INPUT_SIGNAL => {
        self.data.value = value; 
      },
      _ => () 
    }
  }
  
  fn compute(&mut self) { 
    self.data.current_index = (self.data.current_index+1)%self.data.actual_size;
    self.data.buffer[self.data.current_index] = self.data.value;
    if self.data.current_index == 0 {
      let _res = self.tx.send(self.data.clone());
    }
  }

  fn get_output_value(&mut self, _ouput: i32) -> f32 { 
    self.data.value 
  }

}