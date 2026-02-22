use auth_service::authenticate;
use auth_service::auth_utils::models::Credentials;

fn main() {
    let cred = Credentials {
        username: String::from("sherdev"),
        password: String::from("admin123"),
    };

    authenticate(cred);
}
