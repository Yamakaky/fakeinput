use fakeinput::*;

fn main() {
    let conn: Connection = InputConnection::new();
    conn.key_press(Key::B);
    //conn.button_press(MouseButton::ScrollUp);
}
