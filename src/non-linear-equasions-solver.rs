use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;
use plotters::prelude::*;


enum TYPE{
    EQ,
    SYSTEM
}

enum METHOD{
    CHORD,
    NEWTON
}

fn input(n: &mut usize, a: &mut f64, b: &mut f64, e: &mut f64) -> io::Result<()>{
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

        line = lines.next().unwrap()?;
        *e = line.trim().parse().expect("Input is not Number");
        if *e <= 0.0 { panic!("Error should be bigger than 0");}
        Ok(())
    }
    else {
        Err(Error::new(Other,"Unrecognisable input"))
    }

}


fn main()  {

}

