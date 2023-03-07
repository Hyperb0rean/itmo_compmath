use std::io;
use std::io::{BufRead, stdout};
use std::mem::swap;


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


    let mut v_x = vec![0 as f64; n];

    //Bringing matrix to certain form
    let mut flag: bool = false;
    for i in 0..n {
        let mut  sum: f64 = 0.0;
        let mut maximum: f64 = 0.0;
        let mut max_j: usize = 0;
        for j in 0..n {
            sum+= a[i][j].abs();
            maximum = if maximum < a[i][j].abs() {max_j = j; a[i][j].abs()} else {maximum};
        }
        sum-=maximum;
        if maximum < sum{
            println!("Diverges!!!");
            return Ok(());
        }
        else if  maximum>sum{
            flag=true;
        }

        a.swap(i,max_j);
        b.swap(i,max_j);
    }
    if !flag{
        println!("Diverges!!");
        return Ok(());
    }

    println!("Your matrix after diagonalizing:");

    for m in &a {
        println!("{:?}", &m);
    }


    // Calculation
    loop {
        let mut delta: f64 = 0.0;
        for i in 1..=n {
            let mut s: f64 = 0.0;
            for j in 1..i {
                s += a[i-1][j-1] * v_x[j-1];
            }
            for j in (i+1)..=n {
                s += a[i-1][j-1] * v_x[j-1];
            }
            let x: f64 = (b[i-1] - s) / a[i-1][i-1];
            let d: f64 = (x - v_x[i-1]).abs();
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
