//! The `DelayVar` component reads/writes and returns frames of changeble delay in a fixed-size circular buffer.
//! 
//! current_delay = 0 -> no delay (at t);  
//! current_delay = 1 -> read previous frame (at t-1);  
//! current_delay = 2 -> read previous frame (at t-2);  
//! 
//! time of delay = current_delay / sample_rate  
//! max time delay = buffer_size - 1 / sample_rate  
//! current_delay < buffer_size  
use crate::AudioComponent;

#[derive(Debug)]
pub struct DelayVar {
    buffer_delay: Vec<f32>, // circular buffer to store delayed frames.
    index_write: usize, // position in buffer_delay to write.
    index_read: usize, // position in buffer_delay to read.
    buffer_size: usize, // size of the circular buffer (fixed).
    current_delay: usize, // delay for current tick.
}

impl DelayVar {
    /// Returns a `DelayVar` instance with a fixed size specified by buffer_size and an initial_delay.
    /// 
    /// # Examples:
    /// Creating a `DelayVar` instance with a circular buffer size = 3:
    /// ```rust
    /// # use mydsp_rust::delay_var::DelayVar;
    /// # use mydsp_rust::AudioComponent;
    /// let mut delay1 = DelayVar::new(3, 3); // buffer_size (size) = 3, delay = 3
    /// debug_assert_eq!(delay1.tick2x1(1.0,3), 0.0); // first in_frame = 1.0 at postition 1
    /// debug_assert_eq!(delay1.tick2x1(2.0,3), 0.0); // second in_frame = 2.0 at postition 2
    /// debug_assert_eq!(delay1.tick2x1(3.0,3), 0.0); // third in_frame = 3.0 at postition 3
    /// debug_assert_eq!(delay1.tick2x1(4.0,3), 1.0); // 4.0 at position 1
    /// let mut delay2 = DelayVar::new(3, 1); // buffer_size (size) = 3, delay = 1 
    /// debug_assert_eq!(delay2.tick2x1(1.0,1), 0.0); // first in_frame = 1.0 at postition 0, read 0 at pos 2
    /// debug_assert_eq!(delay2.tick2x1(2.0,1), 1.0); // second in_frame = 2.0 at postition 1, read 0 at position 1
    /// debug_assert_eq!(delay2.tick2x1(3.0,2), 1.0); // third in_frame = 3.0 at postition 2, read 1.0 at position 2
    /// debug_assert_eq!(delay2.tick2x1(4.0,1), 3.0); // 4.0 at position 0, read 3.0 at position 2
    /// ```
    /// # Panics
    /// The function panics if `buffer_size` (circular buffer size) is not a positive integer.
    /// ```rust, should_panic
    /// # use mydsp_rust::delay_var::DelayVar;
    /// let delay_panic = DelayVar::new(0, 1);
    /// ```
    pub fn new(buffer_size: usize, current_delay: usize) -> Self {
        if buffer_size <= 0 {
            panic!("!!!Panic: buffer_size must be a positive integer");
        }
        if current_delay > buffer_size {
            panic!("!!!Panic: current_delay must be less than buffer_size");
        }
        DelayVar {
            buffer_delay: vec![0.0; buffer_size],
            index_write: 0,
            index_read: 0,
            buffer_size,
            current_delay,
        }
    }
    pub fn read(&mut self) -> f32 {
        self.index_read = (self.index_write + self.buffer_size - self.current_delay) % self.buffer_size; // update read position
        let delayed_frame = self.buffer_delay[self.index_read]; // read
        delayed_frame
    }
    pub fn get_delay_maxlen(&self) -> usize {
        self.buffer_delay.len()
    }
   pub fn get_delay_len(&self) -> usize {
        self.current_delay
    }

    /// Sets `current_delay` to modify time of delay.
    /// 
    /// # Examples:
    /// Seting a `current_delay` from 0(no delay) to 2:
    /// ```rust
    /// # use mydsp_rust::delay_var::DelayVar;
    /// # use mydsp_rust::AudioComponent;
    /// let mut delay3 = DelayVar::new(3, 3); // buffer_size (size) = 3, no delay
    /// debug_assert_eq!(delay3.tick2x1(1.0,1), 0.0); // first in_frame = 1.0 at postition 0, read 1.0 at pos 0
    /// debug_assert_eq!(delay3.tick2x1(2.0,1), 1.0); // second in_frame = 2.0 at postition 1, read 1.0 at position 0
    /// delay3.set_current_delay(2);
    /// debug_assert_eq!(delay3.tick2x1(3.0,2), 1.0); // third in_frame = 3.0 at postition 2, read 1.0 at position 0
    /// debug_assert_eq!(delay3.tick2x1(4.0,2), 2.0); // 4.0 at position 1, read 2.0 at position 1
    /// ```
    /// # Panics
    /// The function panics if `buffer_size` (circular buffer size) is not a positive integer.
    /// ```rust, should_panic
    /// # use mydsp_rust::delay_var::DelayVar;
    /// let mut delay_panic = DelayVar::new(3, 1);
    /// delay_panic.set_current_delay(4);
    /// ```
    pub fn set_current_delay(&mut self,current_delay: usize) {
        if current_delay > self.buffer_size {
            panic!("!!!Panic: current_delay must be less than buffer_size");
        }
        self.current_delay = current_delay;
    }
}

impl AudioComponent for DelayVar {
    fn tick2x1(&mut self,in_frame: f32,in_delay: usize) -> f32 {
        self.current_delay = in_delay;
        self.index_read = (self.index_write + self.buffer_size - self.current_delay) % self.buffer_size; // update read position
        let delayed_frame = self.buffer_delay[self.index_read]; // read
        self.buffer_delay[self.index_write] = in_frame; // write
        self.index_write = (self.index_write + 1) % self.buffer_size; // update write position
        delayed_frame
    }
}
