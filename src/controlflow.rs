fn main() {
    conditional_assign();
    loops();
}

fn conditional_assign() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn loops() {
    let mut number = 4;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    for numbers in (1..4).rev() {
        println!("{} !", numbers);
    }
    println!("DÉCOLLAGE !!!");
}
