fn main() {
    // Declare a variable and initialize it
    let mut x = 5;
    println!("The value of x is: {}", x);

    // Mutate the value of x
    x = 6;
    println!("The value of x is: {}", x);

    // Declare a constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing allows us to reuse a variable name with a different type
    let x = x + 1; // x is now an i32
    let x = x * 2; // x is now an i32
    println!("The value of x is: {}", x);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    // If statements
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // Loops
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Breaking loop");
            break;
        }
    }

    // While loops
    let mut count = 0;
    while count <= 10 {
        count += 1;
        println!("The value of count is: {}", count);
    }

    // For loops
    for number in (1..4).rev() {
        println!("The value of number is: {}", number);
    }

    // Functions
    let result = add(5, 6);
    println!("The result of the function is: {}", result);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
