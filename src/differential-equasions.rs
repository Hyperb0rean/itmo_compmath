use std::cmp::max;
use std::ffi::c_double;
use std::io;
use std::io::BufRead;
use crate::METHOD::{Euler, Miln, ModEuler};


fn ex1(x: f64, y: f64) -> f64 { y + (1.0 + x) * y.powi(2) }

fn ex2(x: f64, y: f64) -> f64 { x.powi(2) + y.powi(2) }

fn ex3(x: f64, y: f64) -> f64 { 2.0 * x }


#[derive(Clone, Copy)]
enum METHOD {
    Euler,
    ModEuler,
    Miln,
}

fn input(y: &mut f64, e: &mut f64, a: &mut f64, b: &mut f64, f: &mut fn(f64, f64) -> f64) -> io::Result<METHOD> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Choose function:\n
    1) y' = y + (1+x)y^2\n
    2) y' = y^2 + x^2\n
    3) y' = 2x\n
    ");
    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;
    let chosen_function = buffer.trim().parse().expect("Input is not Number");
    match chosen_function {
        1 => *f = ex1,
        2 => *f = ex2,
        3 => *f = ex3,
        _ => panic!("Choose one of the following functions")
    };


    println!("Enter possible error:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *e = buffer.trim().parse().expect("Input is not Number");
    if *e <= 0.0 { panic!("Error should be bigger than 0"); }

    println!("Enter lower boundary:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *a = buffer.trim().parse().expect("Input is not Number");

    println!("Enter upper boundary:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *b = buffer.trim().parse().expect("Input is not Number");
    if *b <= *a { panic!("Lower boundary is LOWER than upper"); }


    println!("Enter y_0 for y_0 = y(x_0):");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *y = buffer.trim().parse().expect("Input is not Number");

    println!("Choose method:\n
    1) Euler\n
    2) Modified Euler\n
    3) Miln");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    let chosen_function = buffer.trim().parse().expect("Input is not Number");
    let method: METHOD = match chosen_function {
        1 => Euler,
        2 => ModEuler,
        3 => Miln,
        _ => panic!("Choose one of this methods")
    };
    Ok(method)
}

fn main() {
    let mut a = 0.0;
    let mut b = 0.0;
    let mut y0 = 0.0;
    let mut e = 0.0;
    let mut f: fn(f64, f64) -> f64 = ex3;
    match input(&mut y0, &mut e, &mut a, &mut b, &mut f) {
        Ok(method) => {
            let mut old_y: f64 = 0.0;
            let mut h = (b - a) / 5.0;
            loop {
                let n: usize = ((b - a) / h).floor() as usize;
                let mut y: Vec<f64> = vec![0.0; n];
                let mut x: Vec<f64> = vec![0.0; n];
                y[0] = y0;
                for i in 0..n {
                    x[i] = a + (i as f64) * h;
                }
                match method {
                    Euler => {
                        for i in 1..n {
                            y[i] = y[i - 1] + h * f(x[i - 1], y[i - 1])
                        }
                    }
                    ModEuler => {
                        for i in 1..n {
                            y[i] = y[i - 1] + h * (f(x[i - 1], y[i - 1]) + f(x[i], y[i - 1] + h * f(x[i - 1], y[i - 1]))) / 2.0;
                        }
                    }
                    Miln => {
                        for i in 1..std::cmp::min(4, n) {
                            y[i] = y[i - 1] + h * (f(x[i - 1], y[i - 1]) + f(x[i], y[i - 1] + h * f(x[i - 1], y[i - 1]))) / 2.0;
                        }


                        for i in 4..n {
                            let mut y_pred = y[i - 4] + 4.0 * h * (2.0 * f(x[i - 3], y[i - 3]) - f(x[i - 2], y[i - 2])) / 3.0 + 2.0 * f(x[i - 1], y[i - 1]);
                            let mut y_corr = y[i - 2] + h * (f(x[i - 2], y[i - 2]) + 4.0 * f(x[i - 1], y[i - 1]) + f(x[i], y_pred)) / 3.0;
                            while (y_corr - y_pred) > e {
                                let mut y_pred = y[i - 4] + 4.0 * h * (2.0 * f(x[i - 3], y[i - 3]) - f(x[i - 2], y[i - 2])) / 3.0 + 2.0 * f(x[i - 1], y[i - 1]);
                                let mut y_corr = y[i - 2] + h * (f(x[i - 2], y[i - 2]) + 4.0 * f(x[i - 1], y[i - 1]) + f(x[i], y_pred)) / 3.0;
                            }
                            y[i] = y_corr;
                        }
                    }
                }

                if (y[n - 1] - old_y).abs() / 3.0 < e {
                    println!("n: {}\nx: {:?}\ny: {:?}\n", n, &x, &y);
                    break;
                } else { h /= 2.0 }
                old_y = y[n - 1];
            }
        }
        Err(e) => panic!("{}", e.to_string())
    };
}
// n: 4
// x: [1.0, 1.25, 1.5, 1.75]
// y: [-1.0, -0.75, -0.62109375, -0.5352687835693359]
// n: 2
// x: [1.0, 1.5]
// y: [-1.0, -0.71875]
// n: 2
// x: [1.0, 1.5]
// y: [-1.0, -0.71875]
