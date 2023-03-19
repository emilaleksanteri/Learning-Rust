fn fib(num: u32) -> i64 {
    if num <= 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }
    else {
        return fib(num - 1) + fib(num - 2);
    };
    
}

fn main() {
    for num in 0..11 {
        println!("fib fib for ({}) is {}", num, fib(num));
    };
}
