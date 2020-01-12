use libc::{c_int, c_uint, c_ulong};
use std::ptr;
use x11::{keysym, xlib, xtest};

pub struct Connection {
    display: *mut xlib::Display,
}

impl Connection {
    pub fn new() -> Connection {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display == ptr::null_mut() {
                panic!("can't open display");
            }
            Connection { display }
        }
    }

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

    fn flush(&self) {
        unsafe {
            let _ = xlib::XFlush(self.display);
        }
    }

    pub fn key_down(&self) {
        self.key_event(true);
        self.flush();
    }

    pub fn key_up(&self) {
        self.key_event(false);
        self.flush();
    }

    pub fn key_press(&self) {
        self.key_event(true);
        self.key_event(false);
        self.flush();
    }

    fn button_up(&self, button: MouseButton) {
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
