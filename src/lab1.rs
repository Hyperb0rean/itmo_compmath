use std::io;
use std::io::{BufRead, stdout};


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();


    //Input variables
    println!("Enter matrix dim:");
    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;
    let n: usize = buffer.trim().parse().expect("Input is not Number");


    let mut a = vec![vec![0 as f64; n]; n];
    println!("Enter matrix coefficients");
    for j in 0..n {
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        a[j] = buffer.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (&a)[j].len() > n { panic!("Line {} is more than dim", j) }
    }


    println!("Your matrix:");
    for m in &a {
        println!("{:?}", &m);
    }

    let mut b = vec![0 as f64; n];
    println!("Enter vector of free coefficients");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    b = buffer.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
    if b.len() > n { panic!("Vector is more than dim") }

    println!("Enter possible error:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    let e: f64 = buffer.trim().parse().expect("Input is not Number");


    println!("Enter max iterations:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    let m: usize = buffer.trim().parse().expect("Input is not Number");

    let mut v_x = vec![0 as f64; n];

    //Bringing matrix to certain form

    


    // Calculation
    for k in 1..m {
        let mut delta: f64 = 0.0;
        for i in 0..n {
            let mut s: f64 = 0.0;
            for j in 0..(i-1) {
                s += a[i][j] * v_x[j];
            }
            for j in i..n {
                s += a[i][j] * v_x[j];
            }
            let x: f64 = (b[i] - s) / a[i][i];
            let d: f64 = (x - v_x[i]).abs();
            if d > delta {
                delta = d;
            }
            v_x[i - 1] = x;
        }
        if delta < e {
            println!("Your answer:");
            println!("{:?}", &v_x);
            return Ok(());
        }
    }
    println!("Diverges!");

    Ok(())
}
