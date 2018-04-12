extern crate hidapi;

mod color;
mod keyboard;

pub use color::Color;
pub use keyboard::Keyboard;
pub use hidapi::HidError;