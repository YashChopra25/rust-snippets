#![allow(dead_code, unused_variable)]

pub mod auth_utils;
mod database;

use auth_utils::{get_user, login, models};
use database::Status;

pub fn authenticate(cred: auth_utils::models::Credentials) {
    // if let database::Status::Connected = database::connect_to_database() {
    //     println!("this is good ");
    //     auth_utils::get_user()
    // }
    if let Status::Connected = database::connect_to_database() {
        println!("this is good ");
        get_user();
        login(cred)
    }
}
mod utils;
//.                 src/utils.rs or src/utils/mod.rs
