/*! This library is developed in the context of the RUST course [5th year course option](https://5tc-rust-cc1d61.gitlabpages.inria.fr/) 
 * ([@INSA Lyon - Département Télécommunications, Services et Usages](https://telecom.insa-lyon.fr/) 
 * && [@Emeraude Research Team](https://team.inria.fr/emeraude/)).
 * 
 * The crate `mydsp-jack` was originally written by Longrui Ma and reproduces the functionalities found in 
 * [the original C++ `mydsp` library](https://github.com/grame-cncm/embaudio/tree/master/examples/teensy/libraries/mydsp).
 *  The crate `mydsp-rust` is an evolution of `mydsp-jack` now developped by Emeraude.
 *
 * # How to use 
 * `mydsp-rust` with [jack bindings for rust](https://github.com/RustAudio/rust-jack) should be added as dependencies
 * in another binary crate.
 * 
 * # Notes
 * ## `impl ProcessHandler for Patch` in user's binary crate
 * Because we want the modules to be connected in a sequential manner rather than [nested](https://github.com/TODO), 
 * `impl ProcessHandler for Patch` (`ProcessHandler` trait defined in `jack-rust`) 
 * cannot be implemented in the `mydsp-rust` library crate, 
 * it can only be implemented in the user's binary crate by defining a new struct `Patch`. (**orphan rule**) 
 * So the final result is that `mydsp-rust` is completely independent of `jack-rust`.
 * 
 * ## #[derive(Debug)]
 * Structs implant `#[derive(Debug)]`, to use `std::fmt formatting` traits, derive from Debug trait.
 * 
 * Usage: `println?("{:?}", all_type)` or `println?("{:#?}", all_type)` to display any type without impl `fmt::Display` manually.
 */


//pub mod dummy;
//pub mod gain;
//#[doc(alias = "PWM")]
//pub mod pwm;
//pub mod noise;
#[doc(alias = "sinetable")]
#[doc(alias = "table")]
pub mod sine_table;
#[doc(alias = "phase")]
pub mod phasor;
#[doc(alias = "sinewave")]
pub mod sine;
pub mod delay;
pub mod echo;
#[doc(alias = "varible_delay")]
#[doc(alias = "delay_varible")]
pub mod delay_var;
pub mod smooth;
pub mod one_zero;
pub mod distortion;
//pub mod am;
//pub mod fm;
// pub mod flanger;
// pub mod ks;

pub trait AudioComponent: Send + Sync{ 
    fn tick(&mut self, in_frame: f32) -> f32 { in_frame }//pass through par default
    fn tick2x1(&mut self, in_frame1: f32, _in_delay: usize) -> f32{ in_frame1 } //used for delay_var
}

pub fn multiply(signals: &[f32]) -> f32 {
    signals.iter().product()
}

pub fn add(signals: &[f32]) -> f32 {
    signals.iter().sum()
}

pub fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}
