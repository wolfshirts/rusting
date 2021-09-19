use std::io;

fn main() {
    println!("Input f to convert fahrenheit to celsius, c for celsius to fahrenheit.");
    let mut which_type = String::new();
    io::stdin()
        .read_line(&mut which_type)
        .expect("Failed to read line.");
    println!("Input temperature: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");
    let temp: f64 = temp.trim().parse().expect("Something went wrong.");

    if which_type == "c\n" {
        let result = temp * 9.0 / 5.0 + 32.0;
        println!("{}c is equal to {}f", temp, result);
    } else {
        let result = (temp - 32.0) * 5.0 / 9.0;
        println!("{}f is equal to {}c", temp, result);
    }
}
