use std::io;

fn main() {
    let mut fibo = String::new();
    println!("Which number would you like?");
    io::stdin()
        .read_line(&mut fibo)
        .expect("There was an error.");
    let fibo: u64 = match fibo.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number.");
            1
        }
    };
    let mut first = 0;
    let mut second = 1;
    let mut result = 0;
    let mut counter = 2;
    if fibo == 0 {
        println!("No 0 fib");
    } else if fibo == 1 {
        println!("fib is 0");
    } else if fibo == 2 {
        println!("fib is 1");
    } else {
        while counter < fibo {
            result = first + second;
            first = second;
            second = result;
            counter += 1;
        }
        println!("The fibo is {}", result);
    }
}
