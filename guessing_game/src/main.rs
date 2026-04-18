fn main() {
    println!("Hello, world!");
    let celsius_temperature: f64 = 32.0;
    let fehrenheit_temperature = celsius_to_fehrenheit(celsius_temperature);
    println!(
        "celsius Temperature: {:.2}, fehrenheit temperature : {:.2}",
        celsius_temperature, fehrenheit_temperature
    );
    let series = fibonacci_series(140);
    for i in 0..series.len() {
        println!("The value at index: {} is: {}", i, series[i]);
    }
}

fn celsius_to_fehrenheit(f: f64) -> f64 {
    (9.0 / 5.0) * f + 32.0
}

fn fibonacci_series(lenth_of_series: u64) -> Vec<u128> {
    if lenth_of_series == 0 {
        println!("The series should have atleast two numbers");
        return vec![];
    } else if lenth_of_series == 1 {
        println!("The series should have atleast two numbers");
        return vec![0];
    }
    let mut series: Vec<u128> = vec![0, 1];
    for x in 2..lenth_of_series {
        let next_number = series[(x - 1) as usize] + series[(x - 2) as usize];
        series.push(next_number);
    }
    series
}
