use crate::synth::dsp::audionode::AudioNode;
use crate::osc::osc::OSCReceiverFactory;

use crate::synth::dsp::inputs::knob::KnobNode;
use crate::synth::dsp::inputs::keyboad::KeyboardNode;

use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;
use crate::synth::dsp::oscillators::baseoscillator::CommonOscillator;
use crate::synth::dsp::oscillators::simple::SinNode;
use crate::synth::dsp::oscillators::simple::SquareNode;
use crate::synth::dsp::oscillators::simple::SawNode;
use crate::synth::dsp::oscillators::rand::RandNode;

use crate::synth::dsp::various::samplehold::SampleHoldNode;
use crate::synth::dsp::various::viz::ScopeNode;

use crate::synth::dsp::filters::moog::MoogFilterNode;

use crate::synth::utils::converters::lfo_frequency_to_voltage;
use crate::synth::engine::synth::Synth;

pub struct SimpleSynth {
    knob:  Box<dyn AudioNode>,
    oscillator: Box<dyn AudioNode>,
    
    lfo:  Box<dyn AudioNode>,
    lfo2:  Box<dyn AudioNode>,
    sh: Box<dyn AudioNode>,
    scope: Box<dyn AudioNode>,
    keyboard: Box<dyn AudioNode>,
    filter: Box<dyn AudioNode>,
}

impl SimpleSynth {

    pub fn new(sample_rate: i32, osc_receiver_factory: OSCReceiverFactory) -> SimpleSynth {
        let mut knob = Box::new(KnobNode::new(&osc_receiver_factory,0.0,String::from("/fader")));
        let mut lfo  = Box::new(SinNode::new(OscillatorMode::LFO,lfo_frequency_to_voltage(1.0),0.90,true));
        let mut lfo2 = Box::new(SinNode::new(OscillatorMode::LFO,lfo_frequency_to_voltage(5.0),0.90,true));
        let mut sh =   Box::new(SampleHoldNode::new());
        let mut oscillator = Box::new(RandNode::new(0.8));
        //let mut oscillator = Box::new(SquareNode::new(OscillatorMode::AUDIO,0.0,0.6,true));//,SawNode::TRI));
        // let mut oscillator = Box::new(SinNode::new(OscillatorMode::AUDIO,0.0,0.1,true));
        let mut keyboard = Box::new(KeyboardNode::new(&osc_receiver_factory));
        let mut scope = Box::new(ScopeNode::new());
        let mut filter = Box::new(MoogFilterNode::new());

        oscillator.set_sample_rate(sample_rate);
        lfo.set_sample_rate(sample_rate);
        filter.set_sample_rate(sample_rate);

        SimpleSynth { 
                oscillator: oscillator,
                lfo: lfo,
                lfo2: lfo2,
                keyboard: keyboard,
                knob: knob,
                scope: scope,
                filter: filter,
                sh: sh
        }
    }

    pub fn compute( &mut self) -> f32 {
        self.keyboard.compute();
        self.knob.compute();
        self.lfo.compute();
        self.lfo2.compute();

        self.oscillator.set_input_value(RandNode::INPUT_TRIGGER, self.keyboard.get_output_value(KeyboardNode::OUTPUT_NOTE_ON));

        // self.oscillator.set_input_value(CommonOscillator::INPUT_FREQ, self.keyboard.get_output_value(KeyboardNode::OUTPUT_FREQ));
        // self.oscillator.set_input_value(CommonOscillator::INPUT_AMP, self.lfo.get_output_value(CommonOscillator::OUTPUT_OSC));
        // self.oscillator.set_input_value(CommonOscillator::INPUT_TRIGGER, self.keyboard.get_output_value(KeyboardNode::OUTPUT_NOTE_ON));
        //self.oscillator.set_input_value(SquareNode::INPUT_RATIO, self.knob.get_output_value(KnobNode::OUTPUT_VALUE));
        self.oscillator.compute();
        
        self.sh.set_input_value(SampleHoldNode::INPUT_SIGNAL, self.oscillator.get_output_value(RandNode::OUTPUT_SIGNAL));
        self.sh.set_input_value(SampleHoldNode::INPUT_TRIGGER, self.lfo2.get_output_value(CommonOscillator::OUTPUT_OSC));
        self.sh.compute();

        self.filter.set_input_value(MoogFilterNode::INPUT_AUDIO, self.oscillator.get_output_value(CommonOscillator::OUTPUT_OSC));
        self.filter.set_input_value(MoogFilterNode::INPUT_CUTOFF,   2.0*self.keyboard.get_output_value(KeyboardNode::OUTPUT_FREQ));
        self.filter.set_input_value(MoogFilterNode::INPUT_RESONANCE, self.sh.get_output_value(SampleHoldNode::OUTPUT_SIGNAL));
        self.filter.compute();

        // Scope
        self.scope.set_input_value(ScopeNode::INPUT_SIGNAL,self.filter.get_output_value(MoogFilterNode::OUPUT_LP));
        // self.scope.set_input_value(ScopeNode::INPUT_SIGNAL,self.sh.get_output_value(SampleHoldNode::OUTPUT_SIGNAL));
        self.scope.compute();
        // Scope

        self.filter.get_output_value(MoogFilterNode::OUPUT_LP)
        
    }

}

impl Synth for SimpleSynth {
    fn compute( &mut self) -> f32 {
        SimpleSynth::compute(self)
    }
}

/*
mod synth;
mod player;
mod osc;

extern crate cpal;
extern crate failure;
extern crate piston_window;

use std::sync::{Arc, Mutex};

use osc::osc::OSC;
use synth::engine::simple::SimpleSynth;
use player::soundsystem::SoundSystem;


fn start() {
    println!("Starting osc receiver.");
    let osc = OSC::new(String::from("127.0.0.1"),6666);
    osc.start();

    println!("Starting sound system.");
    let mut sound_system = SoundSystem::build();
    let synth = Arc::new(Mutex::new(SimpleSynth::new(sound_system.sample_rate(), osc.receiver_factory())));
    
    println!("Starting sound system.");
    let sound_thread = sound_system.start(synth);
    println!("Started.");
    
    let _ = sound_thread.join();
    println!("Stopped.");
}

mod graph;
use crate::synth::dsp::registry::AudioNodeRegistry;
use crate::graph::graph::DspGraph;

fn main() {
    let mut g: DspGraph = DspGraph::new();

    let id1 = String::from("Sin1");
    let id2 = String::from("Sin2");

    g.add_audio_node(&id1, AudioNodeRegistry::SQUARE_LFO);
    g.add_audio_node(&id2, AudioNodeRegistry::SQUARE);

    g.add_link(&id1, 1, &id1, 2);
    match g.add_link(&id1, 0, &id2, 0) {
        Ok(()) => println!("Create: Ok"),
        Err(s) => println!("Erreur: {}",s.as_str())
    }
    // println!("{}",g);
    // match g.remove_node(&id1) {
    //     Ok(()) => println!("Remove: Ok"),
    //     Err(s) => println!("Erreur: {}",s.as_str())
    // }

    println!("{}",g);

    g.compute();
    // start();
}
 */