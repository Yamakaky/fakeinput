use fakeinput::Connection;

fn main() {
    let conn = Connection::new();
    conn.move_mouse(10, 10);
}
