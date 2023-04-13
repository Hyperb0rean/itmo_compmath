use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;

const ACCURACY: f64 =0.001;

enum FUNCTION{
    POLYNOMIAL(u16),
    EXPONENTIAL,
    LOGARITHMIC,
    POWER
}

fn input(n: &mut usize,x: &mut Vec<f64>,y: &mut Vec<f64>) -> io::Result<()>{
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Choose mode: (\"console\" for console input, \"file\" for file input)");

    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;

    if buffer.trim() == "console"{

        println!("Enter number of points:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *n = buffer.trim().parse().expect("Input is not Number");
        if *n<1 || *n>12 {panic!("Number either impossible or too big");}


        *x = vec![0 as f64; *n];
        println!("Enter vector of X values");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *x = buffer.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*x).len() > *n { panic!("Vector is more than number") }

        *y = vec![0 as f64; *n];
        println!("Enter vector of Y values");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *y = buffer.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*y).len() > *n { panic!("Vector is more than number") }

        Ok(())
    }
    else if  buffer.trim() == "file"{
        println!("Enter file path:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        let file = File::open(buffer.trim()).unwrap();
        let mut lines  =io::BufReader::new(file).lines();
        let mut line = lines.next().unwrap()?;
        *n = line.trim().parse().expect("Input is not Number");
        if *n<1 || *n>12 {panic!("Number either impossible or too big");}


        *x = vec![0 as f64; *n];
        line = lines.next().unwrap()?;
        *x = line.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*x).len() > *n { panic!("Vector is more than number") }

        *y = vec![0 as f64; *n];
        line = lines.next().unwrap()?;
        *y = line.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*y).len() > *n { panic!("Vector is more than number") }


        Ok(())
    }
    else {
        Err(Error::new(Other,"Unrecognisable input"))
    }

}

fn approximation_calculation(f: FUNCTION, n: usize, x : &Vec<f64>, y : &Vec<f64>) -> Vec<f64>{
    let mut b:Vec<f64> = vec![];
    let mut matrix:Vec<Vec<f64>> = vec![vec![]];
    match f {
        FUNCTION::POLYNOMIAL(m)=>{
            b = vec![0 as f64; (m+1) as usize];
            matrix = vec![vec![0 as f64; (m+1) as usize]; (m+1) as usize];

            for i in 0..=m {
                for j in 0..n{
                    b[i as usize]+= ((*x)[j as usize].powi(i as i32))*(*y)[j as usize];
                }
            }
            for i in 0..=m {
                for j in 0..=m{
                    let mut  sum:f64 = 0.0;
                    for k in 0..n{
                        sum += x[k].powi((i+j as u16) as i32);
                    }
                    matrix[i as usize][j as usize] = sum;
                }
            }
            println!("Your matrix:");
            for k in &matrix {
                println!("{:?}", &k);
            }

            calculation((m+1) as usize, &mut matrix, &mut b, ACCURACY)
        }
        FUNCTION::EXPONENTIAL => {
            let mut a = vec![0 as f64; 2];
            a
        }
        FUNCTION::LOGARITHMIC => {
            let mut a = vec![0 as f64; 2];
            a
        }
        FUNCTION::POWER =>{
            let mut a = vec![0 as f64; 2];
            a
        }
    }

}

fn calculation(n: usize,a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>, e: f64) -> Vec<f64>{
    let mut v_x = vec![0 as f64; n];
    loop {
        k+=1;
        let mut delta: f64 = 0.0;
        for i in 1..=n {
            let mut s: f64 = 0.0;
            for j in 1..i {
                s += (*a)[i-1][j-1] * v_x[j-1];
            }
            for j in (i+1)..=n {
                s += (*a)[i-1][j-1] * v_x[j-1];
            }
            let x: f64 = ((*b)[i-1] - s) / (*a)[i-1][i-1];
            let d: f64 = (x - v_x[i-1]).abs();
            if d > delta {
                delta = d;
            }
            v_x[i - 1] = x;
        }
        if delta < e {
            break;
        }

    }
    v_x
}

fn main() {
    let mut n:usize = 0;
    let mut x:Vec<f64> = vec![];
    let mut y:Vec<f64> = vec![];

    match input(&mut n, &mut x,&mut y) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };


    println!("{:?}", approximation_calculation(FUNCTION::POLYNOMIAL(2), n, &mut x, &mut y));
}