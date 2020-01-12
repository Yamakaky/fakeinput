use std::default::Default;
use winapi::ctypes::*;
use winapi::shared::minwindef::{UINT, WORD};
use winapi::um::winuser::*;
use crate::*;

pub struct Connection {}

impl Connection {
    fn key_event(&self, pressed: bool) {
        let vkKey: UINT = 0;
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: INPUT_u::default(),
        };
        unsafe {
            let mut flags = KEYEVENTF_SCANCODE;
            if !pressed {
                flags |= KEYEVENTF_KEYUP
            };
            if vkKey >= VK_LEFT as UINT && vkKey <= VK_DOWN as UINT {
                flags |= KEYEVENTF_EXTENDEDKEY;
            }
            *input.u.ki_mut() = KEYBDINPUT {
                time: 0,
                wVk: 0,
                // TODO or MapVirtualKeyW ?
                wScan: MapVirtualKeyA(vkKey, MAPVK_VK_TO_VSC) as WORD,
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

    fn key_down(&self) {
        self.key_event(true);
    }

    fn key_up(&self) {
        self.key_event(false);
    }

    fn key_press(&self) {
        self.key_event(true);
        self.key_event(false);
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
