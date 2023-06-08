use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;
use plotters::prelude::*;


fn polynom(x:f64) -> f64 {2.0*x.powi(3) - 9.0*x.powi(2) -7.0*x +11.0}
fn sinus(x:f64) -> f64 {x.sin()}
fn linear(x:f64) -> f64 {2.0*x}
fn exponent(x:f64) -> f64 {x.exp()}

enum Func{
    
}

#[derive(Clone,Copy)]
enum Method
{
    EqChord,
    Newton(f64),
    Iter(f64)
}


fn left_rectangles(a:f64, b:f64, n:i32, f: fn (f64) -> f64 ) -> f64{
    let h = (b-a)/n as f64;
    let mut sum:f64 = 0.0;
    for i in 1..=n {
        sum+=f(a+h*((i-1) as f64));
    }
    h*sum
}

fn right_rectangles(a:f64,b:f64,n:i32, f: fn (f64) -> f64 ) -> f64{
    let h = (b-a)/n as f64;
    let mut sum:f64 = 0.0;
    for i in 1..=n {
        sum+=f(a+h*(i as f64));
    }
    h*sum
}

fn middle_rectangles(a:f64,b:f64,n:i32, f: fn (f64) -> f64 ) -> f64{
    let h = (b-a)/n as f64;
    let mut sum:f64 = 0.0;
    for i in 1..=n {
        sum+=f(a+h*(i as f64-0.5));
    }
    h*sum
}

fn trapezoid(a:f64,b:f64,n:i32, f: fn (f64) -> f64) -> f64{
    let h = (b-a)/n as f64;
    let mut sum:f64 = a/2.0;
    for i in 1..n {
        sum+=f(a+h*(i as f64));
    }
    sum+=b/2.0;
    h*sum
}

fn simpson(a:f64,b:f64,n:i32, f: fn (f64) -> f64 ) -> f64{
    let h = (b-a)/n as f64;
    let mut sum:f64 = a;
    for i in 1..n {
        sum+=(4*(i%2) + 2*((i+1)%2)) as f64 *f(a+h*(i as f64));
    }
    sum+=b;
    h*sum/3.0
}


fn input(e: &mut f64, a: &mut f64, b: &mut f64, f: &mut fn (f64) -> f64, method: &Method) -> io::Result<()> {

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Choose method:\n
    1) left rectangles\n
    2) right rectangles\n
    3) middle rectangles\n
    4) trapezoids\n
    5) simpson");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    let chosen_function = buffer.trim().parse().expect("Input is not Number");
    match chosen_function {
        1 => *method =  left_rectangles,
        2 => *method =  right_rectangles,
        3 => *method =  middle_rectangles,
        4 => *method =  trapezoid,
        5 => *method =  simpson,
        _ => panic!("Choose one of the following methods")
    };

    println!("Choose function:\n
    1) 2x^3 - 9x^2 -7x +11\n
    2) sin(x)\n
    3) 2x\n
    4) e^x");
    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;
    let chosen_function = buffer.trim().parse().expect("Input is not Number");
    match chosen_function {
        1 => *f = polynom,
        2 => *f = sinus,
        3 => *f = linear,
        4 => *f = exponent,
        _ => panic!("Choose one of the following functions")
    };


    println!("Enter possible error:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *e = buffer.trim().parse().expect("Input is not Number");
    if *e <= 0.0 { panic!("Error should be bigger than 0");}

    println!("Enter lower boundary:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *a = buffer.trim().parse().expect("Input is not Number");

    println!("Enter upper boundary:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *b = buffer.trim().parse().expect("Input is not Number");

    Ok(())
}

fn main() {
    let mut a = 0.0;
    let mut b = 0.0;
    let mut e=0.0;
    let mut f:fn(f64) -> f64 = linear;
    let mut method:Method;
    match input(&mut e, &mut a, &mut b, &mut f, &mut method) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };

}

