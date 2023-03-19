use std::io; // import a library for user input

fn convert_f(far: i32) -> i32 {
    (far - 32) * 5/9
}

fn convert_c(cel: i32) -> i32 {
    cel * 9/5 + 32
}


fn int_input() -> i32 {
    let number = loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("failed to read");
    
        match number.trim().parse() {
            Ok(num) => {break num},
            Err(_) => continue
        };
    };

    return number
}

fn main() {
    println!("Convert Fahrenheit to Celcius type 1");
    println!("Convert Celcius to Fahrenheit type 2");

    let option = int_input();

    if option == 1 {
        let input = int_input();
        let convert = convert_f(input);
        println!("{input} F is {convert} C");
    }

    else if option == 2 {
        let input = int_input();
        let convert = convert_c(input);
        println!("{input} C is {convert} F");
    }
    
}
