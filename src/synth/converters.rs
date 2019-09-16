// Midi to frequency:
// 440*​2^​((x - 69)/​12)

// -1 => 0hz 
//  1 => 20hz
pub fn voltage_to_lfo_frequency(voltage: f32) -> f32 {
  //20.0 * (voltage + 1.0)/2.0
  10.0 * (voltage + 1.0)
}

pub fn lfo_frequency_to_voltage(freq: f32) -> f32 {
  //20.0 * (voltage + 1.0)/2.0
  (freq/10.0) - 1.0 
}



pub fn midi_to_voltage(midi: f32) -> f32 {
  (midi - 1_f32) / 64_f32 - 1_f32
}

pub fn voltage_to_frequency(voltage: f32) -> f32 {
  440_f32 * 2_f32.powf((64_f32 * voltage - 4_f32) / 12_f32)
}

pub fn voltage_to_boolean(voltage: f32) -> bool {
  voltage > 0.0 
}

pub fn boolean_to_voltage(value: bool) -> f32 {
  match value {
    true => 1_f32,
    false => -1_f32
  }
}

pub fn voltage_to_zero_to_one(voltage: f32) -> f32 {
  (voltage+1.0)/2.0
}

pub struct MidiNote {
}

impl MidiNote {

  pub const C0: f32=12_f32;
  pub const CS0: f32=13_f32;
  pub const D0: f32=14_f32;
  pub const DS0: f32=15_f32;
  pub const E0: f32=16_f32;
  pub const F0: f32=17_f32;
  pub const FS0: f32=18_f32;
  pub const G0: f32=19_f32;
  pub const GS0: f32=20_f32;
  pub const A0: f32=21_f32;
  pub const AS0: f32=22_f32;
  pub const B0: f32=23_f32;
  pub const C1: f32=24_f32;
  pub const CS1: f32=25_f32;
  pub const D1: f32=26_f32;
  pub const DS1: f32=27_f32;
  pub const E1: f32=28_f32;
  pub const F1: f32=29_f32;
  pub const FS1: f32=30_f32;
  pub const G1: f32=31_f32;
  pub const GS1: f32=32_f32;
  pub const A1: f32=33_f32;
  pub const AS1: f32=34_f32;
  pub const B1: f32=35_f32;
  pub const C2: f32=36_f32;
  pub const CS2: f32=37_f32;
  pub const D2: f32=38_f32;
  pub const DS2: f32=39_f32;
  pub const E2: f32=40_f32;
  pub const F2: f32=41_f32;
  pub const FS2: f32=42_f32;
  pub const G2: f32=43_f32;
  pub const GS2: f32=44_f32;
  pub const A2: f32=45_f32;
  pub const AS2: f32=46_f32;
  pub const B2: f32=47_f32;
  pub const C3: f32=48_f32;
  pub const CS3: f32=49_f32;
  pub const D3: f32=50_f32;
  pub const DS3: f32=51_f32;
  pub const E3: f32=52_f32;
  pub const F3: f32=53_f32;
  pub const FS3: f32=54_f32;
  pub const G3: f32=55_f32;
  pub const GS3: f32=56_f32;
  pub const A3: f32=57_f32;
  pub const AS3: f32=58_f32;
  pub const B3: f32=59_f32;
  pub const C4: f32=60_f32;
  pub const CS4: f32=61_f32;
  pub const D4: f32=62_f32;
  pub const DS4: f32=63_f32;
  pub const E4: f32=64_f32;
  pub const F4: f32=65_f32;
  pub const FS4: f32=66_f32;
  pub const G4: f32=67_f32;
  pub const GS4: f32=68_f32;
  pub const A4: f32=69_f32;
  pub const AS4: f32=70_f32;
  pub const B4: f32=71_f32;
  pub const C5: f32=72_f32;
  pub const CS5: f32=73_f32;
  pub const D5: f32=74_f32;
  pub const DS5: f32=75_f32;
  pub const E5: f32=76_f32;
  pub const F5: f32=77_f32;
  pub const FS5: f32=78_f32;
  pub const G5: f32=79_f32;
  pub const GS5: f32=80_f32;
  pub const A5: f32=81_f32;
  pub const AS5: f32=82_f32;
  pub const B5: f32=83_f32;
  pub const C6: f32=84_f32;
  pub const CS6: f32=85_f32;
  pub const D6: f32=86_f32;
  pub const DS6: f32=87_f32;
  pub const E6: f32=88_f32;
  pub const F6: f32=89_f32;
  pub const FS6: f32=90_f32;
  pub const G6: f32=91_f32;
  pub const GS6: f32=92_f32;
  pub const A6: f32=93_f32;
  pub const AS6: f32=94_f32;
  pub const B6: f32=95_f32;
  pub const C7: f32=96_f32;
  pub const CS7: f32=97_f32;
  pub const D7: f32=98_f32;
  pub const DS7: f32=99_f32;
  pub const E7: f32=100_f32;
  pub const F7: f32=101_f32;
  pub const FS7: f32=102_f32;
  pub const G7: f32=103_f32;
  pub const GS7: f32=104_f32;
  pub const A7: f32=105_f32;
  pub const AS7: f32=106_f32;
  pub const B7: f32=107_f32;
  pub const C8: f32=108_f32;
  pub const CS8: f32=109_f32;
  pub const D8: f32=110_f32;
  pub const DS8: f32=111_f32;
  pub const E8: f32=112_f32;
  pub const F8: f32=113_f32;
  pub const FS8: f32=114_f32;
  pub const G8: f32=115_f32;
  pub const GS8: f32=116_f32;
  pub const A8: f32=117_f32;
  pub const AS8: f32=118_f32;
  pub const B8: f32=119_f32;
  pub const C9: f32=120_f32;
  pub const CS9: f32=121_f32;
  pub const D9: f32=122_f32;
  pub const DS9: f32=123_f32;
  pub const E9: f32=124_f32;
  pub const F9: f32=125_f32;
  pub const FS9: f32=126_f32;
  pub const G9: f32=127_f32;
  pub const GS9: f32=128_f32;
  pub const A9: f32=129_f32;
  pub const AS9: f32=130_f32;
  pub const B9: f32=131_f32;
  pub const C10: f32=132_f32;
  pub const CS10: f32=133_f32;
  pub const D10: f32=134_f32;
  pub const DS10: f32=135_f32;
  pub const E10: f32=136_f32;
  pub const F10: f32=137_f32;
  pub const FS10: f32=138_f32;
  pub const G10: f32=139_f32;
  pub const GS10: f32=140_f32;
  pub const A10: f32=141_f32;
  pub const AS10: f32=142_f32;
  pub const B10: f32=143_f32;
}
