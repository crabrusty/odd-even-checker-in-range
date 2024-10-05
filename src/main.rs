use std::io;

fn odd_or_even_check_in_range(a:i32,b:i32,c:usize){
    for i in (a..=b).step_by(c) {
        if i % 2 == 0 {println!("{} is even",i);}
        else {println!("{} is odd",i);}
    }
}

fn get_number_input() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("Invalid input. Please enter a valid i32 number."),
        }
    }
}

fn get_non_zero_usize_input() -> usize {
    loop {
        let mut incrementation = String::new();
        println!("Please enter a non-zero positive number for incrementation step (usize):");
        io::stdin()
            .read_line(&mut incrementation)
            .expect("Failed to read line");
        match incrementation.trim().parse::<usize>() {
            Ok(number) => {
                if number != 0 {
                    return number;
                } else {
                    println!("Zero is not allowed. Please enter a non-zero number.");
                }
            }
            Err(_) => println!("Invalid input. Please enter a valid usize number."),
        }
    }
}

fn main() {
    println!("Welcome to odd or even checker in range!");
    println!("Please enter the starting point:");
    let mut number1 = get_number_input();
    println!("Please enter the end point:");
    let mut number2 = get_number_input();
    let mut leanbig = true;
    while leanbig == true {
    if number1 > number2 {
        println!("The starting point must be less than or equal to the end point.");
        println!("Would you like to change:");
        println!("1. The starting point");
        println!("2. The end point");
        println!("3. Both");
        let mut lean = false;
        while lean == false {
            let choice = get_number_input();
        if choice == 1 {println!("Please enter the starting point:"); number1 = get_number_input();lean = true;}
        else if choice == 2 {println!("Please enter the end point:"); number2 = get_number_input();lean = true;}
        else if choice == 3 {
            println!("Please enter the new starting point:");
            number1 = get_number_input();
            println!("Please enter the new end point:");
            number2 = get_number_input();
        }
        else {println!("Please enter a valid number!"); lean = false;}
        }
    }
    else {
        leanbig = false;
    }
}
    let incrementation = get_non_zero_usize_input();
    odd_or_even_check_in_range(number1, number2,incrementation);
}