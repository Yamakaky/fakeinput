use std::default::Default;
use winapi::ctypes::*;
use winapi::shared::minwindef::{UINT, WORD};
use winapi::um::winuser::*;
use crate::*;

pub struct Connection {}

impl Connection {
    fn key_event(&self, key: Key, pressed: bool) {
        let virtual_key = to_virtualkey(key);
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: INPUT_u::default(),
        };
        unsafe {
            let mut flags = KEYEVENTF_SCANCODE;
            if !pressed {
                flags |= KEYEVENTF_KEYUP
            };
            if virtual_key >= VK_LEFT as UINT && virtual_key <= VK_DOWN as UINT {
                flags |= KEYEVENTF_EXTENDEDKEY;
            }
            *input.u.ki_mut() = KEYBDINPUT {
                time: 0,
                wVk: 0,
                // TODO or MapVirtualKeyW ?
                wScan: MapVirtualKeyA(virtual_key, MAPVK_VK_TO_VSC) as WORD,
                dwFlags: flags,
                dwExtraInfo: 0,
            };
            SendInput(
                1,
                &mut input as *mut INPUT,
                std::mem::size_of_val(&input) as c_int,
            );
        }
    }

    fn button_event(&self, button: MouseButton, pressed: bool) {
        //TODO
    }
}

impl InputConnection for Connection {
    fn new() -> Connection {
        Connection {}
    }

    fn key_down(&self, key: Key) {
        self.key_event(key, true);
    }

    fn key_up(&self, key: Key) {
        self.key_event(key, false);
    }

    fn key_press(&self, key: Key) {
        self.key_event(key, true);
        self.key_event(key, false);
    }

    fn button_down(&self, button: MouseButton) {
        self.button_event(button, true);
    }

    fn button_up(&self, button: MouseButton) {
        self.button_event(button, false);
    }

    fn button_press(&self, button: MouseButton) {
        self.button_event(button, true);
        self.button_event(button, false);
    }

    fn move_mouse(&self, x: i32, y: i32) {
    }
}

fn to_virtualkey(key: Key) -> UINT {
    use Key::*;

    match key {
        A => 0x41,
        B => 0x42,
        C => 0x43,
    }
}
