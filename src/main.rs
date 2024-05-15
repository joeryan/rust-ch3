use std::io;
fn main() {
    println!("Enter a number for the fibanaci!");
    let mut count = String::new();
    io::stdin()
        .read_line(&mut count)
        .expect("Failed to read line!");
    let count: u32 = count.trim().parse().expect("Please type a positive number!");
    for num in 0..count { 
        let fib = get_fibonacci(num);
        let place = num + 1;
        println!("{place} - {fib}");
    }
}

fn get_fibonacci(num: u32) -> u32 {
    if num <= 1 {
        return num;
    } else {
        return get_fibonacci(num-1) + get_fibonacci(num-2);
    }
}