fn main() {
    println!("Hello, world!");
    an_other_function(12);
    print_value_with_unite(5, 'h');
    statements();
    println!("The value of five() is {}", five());
}

fn an_other_function(x: u32) {
    println!("Another function");
    println!("The value of x is: {}", x);
}

fn print_value_with_unite(value: i32, unit: char) {
    println!("The value is: {}{}", value, unit);
}

fn statements() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}