use crate::*;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::str::FromStr;

#[no_mangle]
pub extern "C" fn fakeinput_new() -> *mut c_void {
    Box::into_raw(Box::new(Connection::new())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn fakeinput_drop(c_conn: *mut c_void) {
    // TODO check null
    let _ = Box::from_raw(c_conn);
}

#[no_mangle]
pub unsafe extern "C" fn fakeinput_key_down(c_conn: *mut c_void, c_key: *const c_char) {
    let connection = &mut *(c_conn as *mut Connection);
    connection.key_down(from_c_key(c_key));
}

#[no_mangle]
pub unsafe extern "C" fn fakeinput_key_up(c_conn: *mut c_void, c_key: *const c_char) {
    let connection = &mut *(c_conn as *mut Connection);
    connection.key_up(from_c_key(c_key));
}

#[no_mangle]
pub unsafe extern "C" fn fakeinput_move_mouse(c_conn: *mut c_void, x: i32, y: i32) {
    let connection = &mut *(c_conn as *mut Connection);
    connection.move_mouse(x, y);
}

unsafe fn from_c_key(c_key: *const c_char) -> Key {
    let key = CStr::from_ptr(c_key).to_str().unwrap();
    Key::from_str(key).unwrap()
}
