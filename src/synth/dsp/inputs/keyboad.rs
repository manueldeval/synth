
use rosc::{OscPacket,OscType};
use bus::BusReader;

use crate::synth::dsp::audionode::AudioNode;
use crate::osc::osc::OSCReceiverFactory;
use crate::synth::utils::converters::boolean_to_voltage;
use crate::synth::utils::converters::voltage_to_boolean;
use crate::synth::utils::converters::midi_to_voltage;
use crate::synth::utils::converters:: MidiNote;
use crate::synth::dsp::audionode::ConfigSpec;
use crate::synth::dsp::audionode::ConfigType;
use crate::synth::dsp::audionode::ConfigVal;

pub struct KeyboardNode {
  freq: f32,
  on: f32,
  osc_channel: String,
  receiver: BusReader<OscPacket>
}

impl KeyboardNode {
  pub const OUTPUT_FREQ: i32 = 0;
  pub const OUTPUT_NOTE_ON: i32 = 1;
  
  pub fn new(osc_receiver_factory: &OSCReceiverFactory) -> KeyboardNode {
    KeyboardNode { 
      receiver: osc_receiver_factory.create_receiver(), 
      freq:     midi_to_voltage(MidiNote::A4),
      on:       boolean_to_voltage(false),
      osc_channel: String::from("/keyboard")
    }
  }
}

impl AudioNode for KeyboardNode {
  
  fn get_config_spec() -> Vec<ConfigSpec> { 
    vec!(
      ConfigSpec::new(String::from("osc_channel"),ConfigType::StringType)
    )
  }

  fn set_config(&mut self, key: &String, val: &ConfigVal) -> Result<(),String> {
    KeyboardNode::check_key_value_type(key,val)?;
    match key.as_ref() {
      "osc_channel" => val.as_string().map(|s| self.osc_channel = s),
      _ =>  Err(String::from(format!("Config key {} not implemented.",key)))
    }
  }

  fn set_input_value(&mut self, _input: i32, _value: f32) { }
  fn compute(&mut self) {
    let osc_packet = self.receiver.try_recv();
    match osc_packet {
      Ok(OscPacket::Message(msg)) => {
        if msg.addr == self.osc_channel {
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

  fn get_output_value(&self, _ouput: i32) -> f32 { 
    match _ouput {
      KeyboardNode::OUTPUT_FREQ => self.freq,
      KeyboardNode::OUTPUT_NOTE_ON => self.on,
      _ => 0_f32
    } 
  }
}
