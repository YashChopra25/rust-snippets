use auth_service::authenticate;
use auth_service::Credentials

fn main() {
    println!("Hello, world!");
    let cred = Credentials {
        username: String::from("yashh"),
        password: String::from("abcd"),
    };

    // println!("This line is :{:#?}", cred);
    authenticate(cred)
}
