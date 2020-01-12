mod common;
pub use crate::common::*;

#[cfg(feature = "ffi")]
mod ffi;

#[cfg(unix)]
mod x11;
#[cfg(unix)]
pub use crate::x11::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use crate::windows::*;

pub trait InputConnection {
    fn new() -> Self;

    fn key_down(&self);
    fn key_up(&self);
    fn key_press(&self);

    fn button_down(&self, button: MouseButton);
    fn button_up(&self, button: MouseButton);
    fn button_press(&self, button: MouseButton);

    fn move_mouse(&self, x: i32, y: i32);
}
