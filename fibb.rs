use std::io;

fn main() {
    println!("Enter the number of Fibonacci numbers to generate:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    let mut fib_sequence: Vec<u64> = vec![0, 1];
    for i in 2..n {
        let number = fib_sequence[(i - 1) as usize] + fib_sequence[(i - 2) as usize];
        fib_sequence.push(number);
    }

    println!("Fibonacci Sequence:");
    for item in fib_sequence.iter() {
        print_fibonacci(*item);
    }
}

fn print_fibonacci(num: u64) {
    let width = num as usize;
    let mut line = String::with_capacity(width);

    for _ in 0..width {
        line.push('#');
    }

    println!("{}", line);
}