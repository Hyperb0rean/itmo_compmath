use std::io;
use std::io::{BufRead, Error};
use std::fs::File;
use std::io::ErrorKind::Other;


fn input(n: &mut usize, a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>, e: &mut f64) -> io::Result<()>{
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Choose mode: (\"console\" for console input, \"file\" for file input)");

    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;

    if buffer.trim() == "console"{

        println!("Enter matrix dimension:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *n = buffer.trim().parse().expect("Input is not Number");
        if *n<1 || *n>100 {panic!("Matrix dimension either impossible or too big");}

        *a = vec![vec![0 as f64; *n]; *n];
        println!("Enter matrix coefficients");
        for j in 0..*n {
            buffer = String::new();
            handle.read_line(&mut buffer)?;
            a[j] = buffer.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
            if (&*a)[j].len() > *n { panic!("Line {} is more than dim", j) }
        }


        println!("Your matrix:");
        for m in &*a {
            println!("{:?}", &m);
        }

        *b = vec![0 as f64; *n];
        println!("Enter vector of free coefficients");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *b = buffer.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*b).len() > *n { panic!("Vector is more than dimension") }

        println!("Enter possible error:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *e = buffer.trim().parse().expect("Input is not Number");
        if *e <= 0.0 { panic!("Error should be bigger than 0");}
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
        if *n<1 || *n>100 {panic!("Matrix dimension either impossible or too big");}

        *a = vec![vec![0 as f64; *n]; *n];
        for j in 0..*n {
            line = lines.next().unwrap()?;
            a[j] = line.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
            if (&*a)[j].len() > *n { panic!("Line {} is more than dim", j) }
        }

        *b = vec![0 as f64; *n];
        line = lines.next().unwrap()?;
        *b = line.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*b).len() > *n { panic!("Vector is more than dimension") }

        line = lines.next().unwrap()?;
        *e = line.trim().parse().expect("Input is not Number");
        if *e <= 0.0 { panic!("Error should be bigger than 0");}
        Ok(())
    }
    else {
        Err(Error::new(Other,"Unrecognisable input"))
    }

}

fn diagonal_domination(n: usize,a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>){
    let mut flag: bool = false;
    for i in 0..n {
        let mut  sum: f64 = 0.0;
        let mut maximum: f64 = 0.0;
        let mut max_j: usize = 0;
        for j in 0..n {
            sum+= (*a)[i][j].abs();
            maximum = if maximum < (*a)[i][j].abs() {max_j = j; (*a)[i][j].abs()} else {maximum};
        }
        sum-=maximum;
        if maximum < sum{
            panic!("Diverges!!!");
        }
        else if  maximum>sum{
            flag=true;
        }

        (*a).swap(i,max_j);
        (*b).swap(i,max_j);
    }
    if !flag{
        panic!("Diverges!!");
    }


    println!("Your matrix after diagonalizing:");
    for m in &*a {
        println!("{:?}", &m);
    }
}

fn calculation(n: usize,a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>, e: f64){
    let mut v_x = vec![0 as f64; n];
    loop {
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
            println!("Your answer:");
            println!("{:?}", &v_x);
            break;
        }
    }
}

fn main() {

    let mut a:Vec<Vec<f64>> = vec![];
    let mut n:usize = 0;
    let mut b:Vec<f64> = vec![];
    let mut e:f64 = 0.0;
    match input(&mut n,&mut a,&mut b,&mut e) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };

    diagonal_domination(n,&mut a,&mut b);

    calculation(n,&mut a,&mut b,e);
}
