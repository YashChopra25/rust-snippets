#[derive(Debug)]
enum SpreadSheet {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let vect: Vec<SpreadSheet> = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Float(3.14),
        SpreadSheet::Text(String::from("Hello world")),
        SpreadSheet::Int(3),
    ];
    for i in vect {
        match i {
            SpreadSheet::Int(val) => println!("The value is {}", val),
            SpreadSheet::Float(val) => println!("The value is {}", val),
            SpreadSheet::Text(val) => println!("The value is {}", val),
        }
    }
    // println!("The value is {:?}", vect)
}
// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);

//     let mut initial_vec = vec![1, 2, 3, 4, 5];
//     // let fourth_value = &initial_vec[30]
//     // let fourth_value = initial_vec.get(30).unwrap_or(&-1);
//     let fourth_value = match initial_vec.get(30) {
//         Some(val) => *val,
//         None => {
//             println!("THe index is out of bound");
//             -1
//         }
//     };
//     // println!("Hello, world! {:?}\n {:?}", vec, initial_vec);
//     // vec.pop();
//     // println!(
//     //     "Hello, world! {:?}\n {:?} \n {:?}\n {}",
//     //     vec,
//     //     initial_vec,
//     //     vec.get(2),
//     //     fourth_value
//     // );

//     // for i in initial_vec {
//     //     println!("The value is {}", i);
//     // }
//     // for i in &initial_vec {
//     //     println!("The value is {}", i);
//     // }
//     for i in &mut initial_vec {
//         println!("The value is {}", i);
//         *i = *i * 2;
//     }

//     println!("The printing vector is {:?}", initial_vec)
// }
