fn main() {
    println!("Hello, world!");
    let dice: i32 = 2;
    cta(dice);
}

fn cta(dice: i32) -> i32 {
    match dice {
        2 => {
            println!("This is two");
            2
        }
        3 => {
            println!("This is thre");
            3
        }
        other => {
            println!("This is other: {:?}", other);
            dice
        }
    }
}
