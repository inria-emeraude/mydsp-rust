//! The `SineWave` component for generating a sinewave using a static sinetable and a `Phasor` instance.
use crate::AudioComponent;
use crate::sine_table;
use crate::phasor;
use sine_table::SineTable;
use phasor::Phasor;

#[derive(Debug)]
pub struct SineWave {
    sine_table: &'static SineTable,
    phasor: Phasor, 
    gain: f32,
}

impl SineWave {
    /// Returns a `SineWave` instance by referencing a static `SineTable` and moving the `Phasor` instance.
    /// 
    /// # Examples:
    /// Creating a `SineWave` instance of 440Hz:
    /// ```rust
    /// # use mydsp_rust::phasor::Phasor;
    /// # use mydsp_rust::sine_table::SineTable;
    /// # use mydsp_rust::sine::SineWave;
    /// # use mydsp_rust::AudioComponent;
    /// use once_cell::sync::Lazy;
    /// static SINETABLE1: Lazy<SineTable> = Lazy::new(|| SineTable::new(4096));
    /// let sr = 48000.;
    /// let sinewave1 = SineWave::new(&SINETABLE1, sr);
    /// ```
    pub fn new(sine_table: &'static SineTable, sr: f32) -> Self {
        let gain = 0.5;
        /* TODO: allow an initial phase for Sinewave */
        let phasor = Phasor::new(0.0,sr);
        SineWave {
            sine_table,
            phasor,
            gain,
        }
    }
    pub fn set_freq(&mut self, freq: f32) {
         self.phasor.set_freq(freq);
    }
    pub fn set_gain(&mut self, gain: f32) {
         self.gain = gain; 
    }

}

impl AudioComponent for SineWave {
    fn tick(&mut self, in_frame: f32) -> f32 {
        self.sine_table.get_value(self.phasor.tick(in_frame))*self.gain
    }
}
