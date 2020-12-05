#![allow(unused)]

// Rust has constants, basically let but MUST be known at compile time, can be declared in all scopes
const ONE: i32 = 1;

fn mutate_var() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("1) One is always one: {}", ONE)
}

fn shadowing() {
//     Shady stuff if you ask me
//     y = (5 + 1) * 2 = 12
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

//     Also, that kind of stuff allowed:
    let spaces = "   ";
    let spaces = spaces.len();
}

fn type_stuff() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Access like that
    let (x, y, z) = tup;

    // Or like that
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of z is: {}", z);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let all_threes = [3; 5];
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


fn if_is_an_expression() {
    let condition = true;
    // If is a statement here
    let var = if condition { 5 } else { 3 };

    println!("Kind of ternary: {}", var)
}

fn loop_is_an_expression() {
    let mut counter = 0;

    //while(True)
    let result = loop {
        counter += 1;

        if counter == 10 {
            // return
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn conditional_loop() {
    let mut counter = 0;

    //while(True)
    while counter < 10 {
        counter += 1;
    };

    println!("The counter is {}", counter);
}

fn foreach() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn fibo(n: i32, a: i64, b: i64) -> i64 {
    if n == 1 {
        b
    } else {
        fibo(n - 1, b, a + b)
    }
}

fn nth_fibo(n: i32) -> i64 {
    fibo(n, 0, 1)
}

fn main() {
    mutate_var();
    shadowing();
    type_stuff();
    if_is_an_expression();
    loop_is_an_expression();
    conditional_loop();
    foreach();
    println!("7 + 1 = {}", plus_one(7));
    println!("2) One is always one: {}", ONE);

    println!("50th fibo is: {}", nth_fibo(50))
}
