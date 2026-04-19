#![allow(dead_code, unused_variable)]
// #[derive(Debug)]

// FIRST WAY TO DO IT:
// pub struct Credentials {
//     pub username: String,
//     pub password: String,
// }

// enum Status {
//     Connected,
//     INTRUPRUPTED,
// }

// fn connect_to_database() -> Status {
//     Status::Connected
// }

// fn get_user() {}

// fn login(cred: Credentials) {
//     //try to login
//     get_user()
// }
// pub fn authenticate(cred: Credentials) {
//     if let Status::Connected = connect_to_database() {
//         println!("this is good {:?}",)
//     }
// }

//SECOND WAY TO DO IT:

mod database {

    pub enum Status {
        Connected,
        INTRUPRUPTED,
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }
}

mod auth_utils {
    fn get_user() {}

    pub fn login(cred: models::Credentials) {
        //try to login
        get_user()
    }
    pub mod models {
        pub struct Credentials {
            pub username: String,
            pub password: String,
        }
    }
}

pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        println!("this is good ")
    }
}
