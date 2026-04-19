fn main() {
    println!("Hello, world!");

    let config = Some(3_u8); // can be written in 3u8
    // match config {
    //     Some(max) => println!("This is the given value {max}"),
    //     None => (),
    // }

    if let Some(max) = config {
        println!("This is the given value {max}")
    } else {
        println!("THis is an else state")
    }
}
