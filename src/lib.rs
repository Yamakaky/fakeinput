mod common;
pub use crate::common::*;

mod keys;
pub use crate::keys::*;

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

    fn key_down(&self, key: Key);
    fn key_up(&self, key: Key);
    fn key_press(&self, key: Key);

    fn button_down(&self, button: MouseButton);
    fn button_up(&self, button: MouseButton);
    fn button_press(&self, button: MouseButton);

    fn move_mouse(&self, x: i32, y: i32);
}
