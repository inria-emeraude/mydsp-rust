//! The `Echo` component includes `feedback` control and  control, using a fixed delay from the `Delay` instance.
//! 
//! Returns output = input + delayed input * feedback
//! 
//! ##  Using `Option<f32>` for default values
//! Rust does not have a null type, so `Option<f32>` is used here, which contains `Some()` and `None`.  
//! Without `phase: impl Into<Option<f32>>` and `phase.into().unwrap_or(0.0)`, this usage would not be possible:
//! ```rust
//! # use mydsp_rust::delay::Delay;
//! # use mydsp_rust::echo::Echo;
//! let nframes_delay:usize = 48000/2; // delay of 0.5s if sample rate = 48000Hz
//! let echo0 = Echo::new(nframes_delay, None);
//! ```
//! 
//! **memo**:
//! To achieve using default parameters, I have thought about other methods like using Default trait, 
//! builder pattern or simply exposing fields which contain default values like `pub phase`, 
//! but `phase: impl Into<Option<f32>>` seems to be the most elegant.

use crate::AudioComponent;
use crate::delay_var::DelayVar;

#[derive(Debug)]
pub struct Echo {
    fifo: DelayVar, //array for storing samples
    delay_max: usize, //maximum size of the FIFO
    delay: usize,  // effective delay (can be less than fifo size)
    feedback: f32, // feedback control
}

impl Echo {
    /// Returns a `Echo` instance with a fixed size specified by nframes_delay.
    /// 
    /// # Examples:
    /// Creating an Echo of 0.5 s
    /// ```rust
    /// # use mydsp_rust::delay::Delay;
    /// # use mydsp_rust::echo::Echo;
    /// let nframes_delay:usize = 48000/2; // delay of 0.5s if sample rate = 48000Hz
    /// // output = input +  delayed input * feedback(0.6))
    /// let echo1 = Echo::new(nframes_delay, 0.6,); 
    /// ```
    /// # Panics
    /// The function panics if not 0.0<=`feedback`<=1.0, .
    /// ```rust, should_panic
    /// # use mydsp_rust::delay::Delay;
    /// # use mydsp_rust::echo::Echo;
    /// # let nframes_delay:usize = 48000/2; // delay of 0.5s if sample rate = 48000Hz
    /// // output = input + delayed input * feedback(2.0))
    /// let echo1 = Echo::new(nframes_delay, 2.0);
    /// ```
    pub fn new(nframes_delay: usize, feedback: impl Into<Option<f32>>) -> Self {
        let feedback = feedback.into().unwrap_or(0.0);
        if !(0.0..=1.0).contains(&feedback) {
            panic!("!!!Echo panic: Feedback must be between 0.0 and 1.0");
        }
        let delay = nframes_delay; //by default, max_delay echo
        Echo {
            fifo: DelayVar::new(nframes_delay,nframes_delay),
            delay_max: delay,
            delay,
            feedback,
        }
    }
    pub fn get_max_delay(&mut self) -> usize{
        self.delay_max
    }
    pub fn set_feedback(&mut self, feedback: f32) {
        if !(0.0..=1.0).contains(&feedback) {
            panic!("!!!Echo panic: Feedback must be between 0.0 and 1.0");
        }
        self.feedback = feedback;
    }
   pub fn set_delay(&mut self, delay: usize) {
        if delay > self.delay_max {
            panic!("!!!Echo panic: delay must be less than {}", self.fifo.get_delay_maxlen());
        }
        self.delay = delay;
        self.fifo.set_current_delay(delay);
    }

}

impl AudioComponent for Echo {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let output = in_frame + self.feedback * self.fifo.read();
        self.fifo.tick2x1(output,self.delay);
        output
    }
}
