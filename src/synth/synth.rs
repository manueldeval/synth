use crate::synth::audionode::AudioNode;
use crate::osc::osc::OSCReceiverFactory;

use crate::synth::osc::KnobNode;
use crate::synth::osc::KeyboardNode;

use crate::synth::baseoscillator::OscillatorMode;
use crate::synth::baseoscillator::CommonOscillator;

use crate::synth::oscillators::SinNode;
use crate::synth::oscillators::SquareNode;
use crate::synth::oscillators::SawNode;

use crate::synth::viz::ScopeNode;
use crate::synth::converters::lfo_frequency_to_voltage;


pub struct Synth {
    knob:  Box<dyn AudioNode + Send>,
    oscillator: Box<dyn AudioNode + Send>,
    lfo:  Box<dyn AudioNode + Send>,
    keyboard: Box<dyn AudioNode + Send>,
    scope: Box<ScopeNode>
}

impl Synth {

    pub fn new(sample_rate: i32, osc_receiver_factory: OSCReceiverFactory) -> Synth {
        let mut knob = Box::new(KnobNode::new(&osc_receiver_factory,0.0,String::from("/fader")));
        let mut lfo = Box::new(SinNode::new(OscillatorMode::LFO,lfo_frequency_to_voltage(1.0),0.0,true));
        let mut oscillator = Box::new(SawNode::new(OscillatorMode::AUDIO,0.0,0.1,true,SawNode::TRI));
        // let mut oscillator = Box::new(SinNode::new(OscillatorMode::AUDIO,0.0,0.1,true));
        let mut keyboard = Box::new(KeyboardNode::new(&osc_receiver_factory));
        let mut scope = Box::new(ScopeNode::new());

        oscillator.configure(sample_rate);
        lfo.configure(sample_rate);
        
        Synth { 
            oscillator: oscillator,
            lfo: lfo,
            keyboard: keyboard,
            knob: knob,
            scope: scope
        }
    }

    pub fn compute(&mut self) -> f32 {
        self.keyboard.compute();
        self.lfo.compute();
        self.knob.compute();

        self.oscillator.set_input_value(CommonOscillator::INPUT_FREQ, self.keyboard.get_output_value(KeyboardNode::OUTPUT_FREQ));
        self.oscillator.set_input_value(CommonOscillator::INPUT_AMP, self.lfo.get_output_value(CommonOscillator::OUTPUT_OSC));
        self.oscillator.set_input_value(CommonOscillator::INPUT_TRIGGER, self.keyboard.get_output_value(KeyboardNode::OUTPUT_NOTE_ON));
        self.oscillator.set_input_value(SquareNode::INPUT_RATIO, self.knob.get_output_value(KnobNode::OUTPUT_VALUE));

        self.oscillator.compute();
        
        // Scope
        self.scope.set_input_value(ScopeNode::INPUT_SIGNAL,self.oscillator.get_output_value(CommonOscillator::OUTPUT_OSC));
        self.scope.compute();
        // Scope
        
        self.oscillator.get_output_value(CommonOscillator::OUTPUT_OSC)
    }

}