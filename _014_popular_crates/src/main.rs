use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    id: u32,
    is_deleted: bool,
}
#[derive(Debug, Serialize, Deserialize)]
struct NewUserRequest {
    name: String,
    id: u32,
}
impl From<NewUserRequest> for User {
    fn from(request: NewUserRequest) -> Self {
        Self {
            name: request.name,
            id: request.id,
            is_deleted: false,
        }
    }
}
fn handle_request(json_request: &str) {
    match serde_json::from_str::<NewUserRequest>(json_request) {
        Ok(good_request) => {
            let new_user = User::from(good_request);
            println!("Made a new user! {new_user:#?}");
            println!(
                "Serialized back into JSON: {:#?}",
                serde_json::to_string(&new_user)
            );
        }
        Err(e) => {
            println!("Got an error from {json_request}: {e}");
        }
    }
}
fn main() {
    let good_json_request = r#"
    {
    "name": "BillyTheUser",
    "id": 6876
    }
    "#;
    let bad_json_request = r#"
    {
    "name": "BobbyTheUser",
    "idd": "6877"
    }
    "#;
    handle_request(good_json_request);
    handle_request(bad_json_request);
}
