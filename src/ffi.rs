use crate::*;
use std::ffi::c_void;

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
pub unsafe extern "C" fn fakeinput_key_down(c_conn: *mut c_void) {
    let connection = &mut *(c_conn as *mut Connection);
    connection.key_down();
}

#[no_mangle]
pub unsafe extern "C" fn fakeinput_key_up(c_conn: *mut c_void) {
    let connection = &mut *(c_conn as *mut Connection);
    connection.key_up();
}

#[no_mangle]
pub unsafe extern "C" fn fakeinput_move_mouse(c_conn: *mut c_void, x: i32, y: i32) {
    let connection = &mut *(c_conn as *mut Connection);
    connection.move_mouse(x, y);
}
