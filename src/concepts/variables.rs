fn main() {
    mutability();
    constant();
    shadowing();
    float();
    boolean();
    char();
    tuple();
    array();
}

fn mutability() {
    let mut x = 5;
    println!("x has the value {}", x);
    x = 6;
    println!("x has the value {}", x);
}

fn constant() {
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("The constant is {}", THREE_HOURS_IN_SECONDS);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est {}", x);
    }

    println!("La valeur de x est {}", x);
}

fn float() {
    let x = 2.0;
    let y: f32 = 3.0;
    println!("La valeur de x est {}", x);
    println!("La valeur de y est {}", y);
}

fn boolean() {
    let t = true;
    let f: bool = false;
    println!("La valeur de t est {}", t);
    println!("La valeur de f est {}", f);
}

fn char() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("La valeur de c est {}", c);
    println!("La valeur de z est {}", z);
    println!("La valeur de heart_eyed_cat est {}", heart_eyed_cat);
}

fn tuple() {
    let tup:(i32, f64, u8) = (-500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is {}", y);
    println!("The value of y is {}", tup.1);
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    println!("The value of a[0] is {}", a[0]);
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The value of 5th month is {}", months[4]);
    let b = [3; 5];
    println!("The value of the last element of b is {}", b[b.len() - 1]);
}