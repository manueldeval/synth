
use super::audionode::AudioNode;
use super::super::osc::osc::OSCReceiverFactory;
use rosc::{OscPacket,OscType};
use bus::BusReader;
use super::converters::{boolean_to_voltage, voltage_to_boolean, midi_to_voltage, MidiNote};

pub struct KeyboardNode {
  freq: f32,
  on: f32,
  receiver: BusReader<OscPacket>
}

impl KeyboardNode {
  pub const OUTPUT_FREQ: i32 = 0;
  pub const OUTPUT_NOTE_ON: i32 = 1;
  
  pub fn new(osc_receiver_factory: &OSCReceiverFactory) -> KeyboardNode {
    KeyboardNode { 
      receiver: osc_receiver_factory.create_receiver(), 
      freq:     midi_to_voltage(MidiNote::A4),
      on:       boolean_to_voltage(false) 
    }
  }
}

impl AudioNode for KeyboardNode {
  
  fn set_input_value(&mut self, _input: i32, _value: f32) { }
  fn compute(&mut self) {
    let osc_packet = self.receiver.try_recv();
    match osc_packet {
      Ok(OscPacket::Message(msg)) => {
        // println!("OSC address: {}", msg.addr);
        if msg.addr.eq("/keyboard") {
          match msg.args {
            Some(args) => {
              if args.len() == 2 {
                match args[0] {
                  OscType::Float(value) => self.freq = midi_to_voltage(value),
                  _ => ()
                };
                match args[1] {
                  OscType::Float(value) => self.on = value,
                  _ => ()
                };
              }
            }
            None => (),
            // None => println!("No arguments in message."),
          }
        }
      }
      _ => ()
    }
  }

  fn get_output_value(&mut self, _ouput: i32) -> f32 { 
    match _ouput {
      KeyboardNode::OUTPUT_FREQ => self.freq,
      KeyboardNode::OUTPUT_NOTE_ON => self.on,
      _ => 0_f32
    } 
  }
}

pub struct KnobNode {
  value: f32,
  receiver: BusReader<OscPacket>,
  osc_name: String
}

impl KnobNode {
  pub const OUTPUT_VALUE: i32 = 0;
  
  pub fn new(osc_receiver_factory: &OSCReceiverFactory,value: f32, osc_name: String) -> KnobNode {
    KnobNode { 
      receiver: osc_receiver_factory.create_receiver(), 
      value: value,
      osc_name: osc_name
    }
  }
}

impl AudioNode for KnobNode {
  
  fn set_input_value(&mut self, _input: i32, _value: f32) { }
  fn compute(&mut self) {
    let osc_packet = self.receiver.try_recv();
    match osc_packet {
      Ok(OscPacket::Message(msg)) => {
        if msg.addr.eq(self.osc_name.as_str()) {
          match msg.args {
            Some(args) => {
              if args.len() == 1 {
                match args[0] {
                  OscType::Float(value) => self.value = value,
                  _ => ()
                }
              }
            }
            None => (),
          }
        }
      }
      _ => ()
    }
  }

  fn get_output_value(&mut self, _ouput: i32) -> f32 { 
    self.value
  }
}

  // let mut rx = osc.create_receiver();
    // let mut i: i64=0;
    // loop {
    //     i+=1;
    //     match rx.try_recv() {
    //         Ok(OscPacket::Message(msg)) => {
    //             println!("OSC address: {}", msg.addr);
    //             match msg.args {
    //                 Some(args) => {
    //                     println!("OSC arguments: {:?}", args);
    //                 }
    //                 None => println!("No arguments in message."),
    //             }
    //         }
    //         Ok(OscPacket::Bundle(bundle)) => {
    //             println!("OSC Bundle: {:?}", bundle);
    //         }
    //         Err(..) => {
    //             // println!("ERROR :(");
    //         },
    //     }
    //     if i==1_000_000_000 {
    //         break;
    //     }
    // }