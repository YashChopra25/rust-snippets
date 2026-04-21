// // this is an example for error handling in rust. Recoverable errors

// fn main() {
//     // let result = divide(10, 0).unwrap_or(-1);
//     let result = match divide(10, 0) {
//         Ok(num) => num,
//         Err(err) => {
//             println!("Error {err}");
//             -1
//         }
//     };

//     println!("Result: {result}")
// }
// // without error handling,and the result
// // fn divideold(x: i32, y: i32) -> i32 {
// //     x / y
// // }

// // Result<Success,Error>
// fn divide(x: i32, y: i32) -> Result<i32, String> {
//     if y == 0 {
//         return Err(String::from("Please do not divide by zero"));
//     }
//     Ok(x / y)
// }

use core::error;
use std::{fs::File, io::ErrorKind};
fn main() {
    let greeting_file_result = File::open("./yash.txt");
    print!("{:?}", greeting_file_result);
    let file = match greeting_file_result {
        Ok(file_ext) => {
            println!("FIle doung: {:?}", file_ext);
            file_ext
        }
        Err(er) => match er.kind() {
            ErrorKind::NotFound => match File::create("./yash.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
