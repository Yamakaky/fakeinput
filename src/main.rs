use fakeinput::*;

fn main() {
    let conn: Connection = InputConnection::new();
    conn.button_press(MouseButton::ScrollUp);
    conn.button_press(MouseButton::ScrollUp);
}
