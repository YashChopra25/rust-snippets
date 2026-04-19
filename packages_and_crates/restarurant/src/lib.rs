mod front_of_house {

    pub mod hosting {
        fn add_to_waitlist() {}
        pub fn seat_to_table() {
            println!(" Seat to table called")
        }
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {
            crate::front_of_house::hosting::seat_to_table();
            super::hosting::seat_to_table()
        }
        fn take_payment() {}
    }
}

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Mango"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    front_of_house::serving::serve_order();

    let dood = back_of_house::Breakfast::summer("Rye");
    let dood2 = back_of_house::Breakfast::summer("Yashhhh");

    println!("this is back_of_housr {:#?}\n, {:#?}", dood, dood2)
}
