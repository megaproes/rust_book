// In practice, the take() function is often used as a convenience method to quickly
// turn a Some into a None without having to do any pattern matching. The following
// example shows just how short your code can be when using this function:

use std::time::Duration;
struct UserState {
    username: String,
    connection: Option<Connection>, // We could have the Connection struct hold
                                    // the state of the connection, but another
                                    // way to do it is to wrap it in an Option. In
                                    // this case, Some represents a connected
                                    // state, and None, a nonconnected state.
}
struct Connection {
    url: String,
    timeout: Duration,
}
impl UserState {
    fn is_connected(&self) -> bool {
        self.connection.is_some()
    }
    fn connect(&mut self, url: &str) {
        self.connection = Some(Connection {
            url: url.to_string(),
            timeout: Duration::from_secs(3600),
        });
    }
    fn disconnect(&mut self) {
        self.connection.take(); // To disconnect, just take() the
                                // value and do nothing with it,
                                // leaving None in its place.
    }
}
fn main() {
    let mut user_state = UserState {
        username: "Mr. User".to_string(),
        connection: None,
    };
    user_state.connect("someurl.com");
    println!("Connected? {}", user_state.is_connected());
    user_state.disconnect();
    println!("Connected? {}", user_state.is_connected());
}
