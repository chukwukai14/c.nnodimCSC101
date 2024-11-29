use std::io;

// Given the values of a, b and c, find the roots of the quadratic equation using rust program

fn main() {
    
    let mut input1 = String::new();
    println!("\nInput your value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("\nInput your value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let mut input3 = String::new();
    println!("\nInput your value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let determinant = (b * b) - (4.0 * a * c);

    let roots_plus = (-b + determinant.sqrt()) / (2.0 * a);  // Formula for positive part of the determinant
    let roots_minus = (-b - determinant.sqrt()) / (2.0 * a);  // Formula for negative part of the determinant

    if determinant > 0.0{
        println!("\nSince determinant is positive, there are two distinct real roots.");
    }
    else if determinant < 0.0{
        println!("\nSince determinant is negative, there are two distinct imaginary roots.");
    }
    else{
        println!("\nSince determinant is zero, there are one distinct real roots.\nlet us proceed to find the root of the quadratic equation");
        println!("\nThe roots of the quadratic equation are {}. {}", roots_plus, roots_minus);
    }
}
