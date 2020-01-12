#![allow(clippy::cast_lossless)]

use libc::{c_int, c_uint, c_ulong};
use std::ptr;
use x11::xlib::*;
use x11::{keysym, xlib, xtest};

use crate::InputConnection;
use crate::MouseButton;

pub struct Connection {
    display: *mut xlib::Display,
}

impl Connection {
    fn key_event(&self, pressed: bool) {
        unsafe {
            let keycode = xlib::XKeysymToKeycode(self.display, keysym::XK_A as c_ulong) as c_uint;

            assert_ne!(keycode, xlib::NoSymbol as c_uint, "unknown symbol");
            let ret = xtest::XTestFakeKeyEvent(
                self.display,
                keycode,
                if pressed { xlib::True } else { xlib::False },
                xlib::CurrentTime,
            );
            assert_ne!(ret, 0, "key send failure");
        }
    }

    fn button_event(&self, button: MouseButton, pressed: bool) {
        use MouseButton::*;

        unsafe {
            let button_id = match button {
                Left => Button1,
                Middle => Button2,
                Right => Button3,
                ScrollUp => Button4,
                ScrollDown => Button5,
            };
            let ret = xtest::XTestFakeButtonEvent(
                self.display,
                button_id,
                if pressed { xlib::True } else { xlib::False },
                xlib::CurrentTime,
            );
            assert_ne!(ret, 0, "key send failure");
        }
    }

    fn flush(&self) {
        unsafe {
            let _ = xlib::XFlush(self.display);
        }
    }
}

impl InputConnection for Connection {
    fn new() -> Connection {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                panic!("can't open display");
            }
            Connection { display }
        }
    }

    fn key_down(&self) {
        self.key_event(true);
        self.flush();
    }

    fn key_up(&self) {
        self.key_event(false);
        self.flush();
    }

    fn key_press(&self) {
        self.key_event(true);
        self.key_event(false);
        self.flush();
    }

    fn button_down(&self, button: MouseButton) {
        self.button_event(button, true);
        self.flush();
    }

    fn button_up(&self, button: MouseButton) {
        self.button_event(button, false);
        self.flush();
    }

    fn button_press(&self, button: MouseButton) {
        self.button_event(button, true);
        self.button_event(button, false);
        self.flush();
    }

    fn move_mouse(&self, x: i32, y: i32) {
        unsafe {
            //TODO : issue
            let ret = XTestFakeRelativeMotionEvent(
                self.display,
                -1,
                x as c_int,
                y as c_int,
                xlib::CurrentTime,
            );
            assert_ne!(ret, 0, "mouse move failure");
            let _ = xlib::XFlush(self.display);
        }
    }
}

extern "C" {
    fn XTestFakeRelativeMotionEvent(
        _1: *mut xlib::Display,
        _2: c_int,
        _3: c_int,
        _4: c_int,
        _5: c_ulong,
    ) -> c_int;
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            let ret = xlib::XCloseDisplay(self.display);
            assert_eq!(ret, 0, "error closing the display");
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new()
    }
}
