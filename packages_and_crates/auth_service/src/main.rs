use auth_service::auth_utils::models::Credentials;
use auth_service::authenticate;

fn main() {
    println!("Hello, world!");
    let cred = Credentials {
        username: String::from("yashh"),
        password: String::from("abcd"),
    };
    // println!("This line is :{:#?}", cred);
    authenticate(cred)
}
