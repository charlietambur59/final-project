fn main() {
    let x = 5;

    // Basic syntax
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        3 => println!("x is 3"),
        _ => println!("x is something else"),
    }

    // Matching on multiple values
    match x {
        1 | 2 => println!("x is 1 or 2"),
        3 | 4 => println!("x is 3 or 4"),
        _ => println!("x is something else"),
    }

    // Matching on a range of values
    match x {
        1 | 5 => println!("x is between 1 and 5"),
        6 | 10 => println!("x is between 6 and 10"),
        _ => println!("x is something else"),
    }

    // Matching on a pattern
    let y = Some(5);
    match y {
        Some(n) if n > 0 => println!("y is a positive number"),
        Some(n) if n < 0 => println!("y is a negative number"),
        Some(_) => println!("y is zero"),
        None => println!("y is None"),
    }

    // Matching on a type
    let z = "hello";
    match z {
        "hello" => println!("z is a string"),
        _ => println!("z is something else"),
    }
     let x = "no";
    match x {
        "no" => println!("no is a string"),
        _ => println!("no is something else"),
    }
}
