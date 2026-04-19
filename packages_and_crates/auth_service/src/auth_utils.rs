pub fn get_user() {
    println!("The get user function is called:")
}

pub fn login(cred: models::Credentials) {
    //try to login
    get_user()
}
pub mod models;
