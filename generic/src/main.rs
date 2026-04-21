struct Point<T, U> {
    a: T,
    b: U,
}
impl<T: std::fmt::Debug, U: std::fmt::Debug> Point<T, U> {
    fn new(a: T, b: U) -> Self {
        Self { a, b }
    }
    fn display(&self) {
        println!("Point: {:?}, {:?}", self.a, self.b);
    }
}
fn main() {
    println!("Hello, world!");
    // let list = vec![12, 4, 2, 1, 53, 23];
    let list = vec![1.2, 0.4, 2.64, 1.231, 53.4, 2.3];
    println!("The largest is {:?}", largest_generic(&list));
    let point_a = Point::new(1, 2);
    let point_b = Point::new(1.2, "hello");
    point_a.display();
    point_b.display();
}

fn _largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for current in list {
        if largest < current {
            largest = current
        }
    }
    largest
}

fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for current in list {
        if current > largest {
            largest = current;
        }
    }
    largest
}
