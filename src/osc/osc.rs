
use rosc::OscPacket;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use bus::{Bus,BusReader};

#[derive(Clone)]
pub struct OSCReceiverFactory {
  osc_bus: Arc<Mutex<Bus<OscPacket>>>
}

impl OSCReceiverFactory {
  pub fn create_receiver(&self) ->  BusReader<OscPacket> {
    self.osc_bus.lock().unwrap().add_rx()
  }
}

pub struct OSC {
  address: String,
  port: u16,
  osc_bus: Arc<Mutex<Bus<OscPacket>>>
}

impl OSC {
  pub fn new(address: &String, port: u16) -> OSC {
    OSC {address: address.clone(),port: port, osc_bus: Arc::new(Mutex::new(Bus::new(100)))}
  }

  fn start_internal(bus: Arc<Mutex<Bus<OscPacket>>>,address: &String , port: u16) -> JoinHandle<()> {
    let address_and_port = format!("{}:{}", address, port); 
    let addr = match SocketAddrV4::from_str(&address_and_port) {
        Ok(addr) => addr,
        Err(_) => panic!("Bad ip:port format: {}",address_and_port)
    };

    thread::spawn(move || {
      
      let sock = UdpSocket::bind(addr).unwrap();
      println!("Listening to {}", addr);
      let mut buf = [0u8; rosc::decoder::MTU];

      loop {
          match sock.recv_from(&mut buf) {
              Ok((size, addr)) => {
                  println!("Received packet with size {} from: {}", size, addr);
                  let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                  bus.lock().unwrap().broadcast(packet);
              }
              Err(e) => {
                  println!("Error receiving from socket: {}", e);
                  ()
              }
          }
      }
    })
  }

  pub fn receiver_factory(&self) -> OSCReceiverFactory {
    OSCReceiverFactory { osc_bus: self.osc_bus.clone() }
  }

  pub fn start(&self) -> JoinHandle<()> {
    OSC::start_internal(self.osc_bus.clone(),&self.address,self.port)
  }
  
}


