use std::io;

fn even_or_odd_check(n:i32){
    if n % 2 == 0 {println!("even");}
    else {println!("odd");}
}
fn main() {
    let mut number = String::new();
    println!("Write a number to check if it is odd or even:");
    io::stdin()
    .read_line(&mut number)
    .expect("Can't read line");
    let number = number.trim().parse().expect("please enter a valid number");
    even_or_odd_check(number);
}
