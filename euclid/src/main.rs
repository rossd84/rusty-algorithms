// Euclid's algorithm finds the greatest common denominator of 2 numbers.

fn main() {
    let a = 50;
    let b = 10;

    let gcd = euclid_recursive(a, b);
    println!("The GCD of {} and {} is {}", a, b, gcd);
}

// Traditional version
#[allow(dead_code)]
fn euclid_traditional(mut a: i64, mut b: i64) -> i64 {
    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }

    return a;
}

// Iterative version
#[allow(dead_code)]
fn euclid_iterative(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

// Recursive version
fn euclid_recursive(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        euclid_recursive(b, a % b)
    }
}
