use std::arch::x86_64::_mm_extract_ps;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, ChartContext};
use plotters::coord::types::RangedCoordf64;
use plotters::drawing::{ IntoDrawingArea};
use plotters::element::PathElement;
use plotters::prelude::{BLACK, Cartesian2d, Color, IntoFont, LineSeries, RED, WHITE};
use plotters::style::{BLUE, GREEN};
use crate::METHOD::{GAUSS, LAGRANGE, NEWTON};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[derive(Clone,Copy)]
enum METHOD{
    LAGRANGE,
    NEWTON,
    GAUSS
}

fn input(n: &mut usize,val: &mut f64,x: &mut Vec<f64>,y: &mut Vec<f64>) -> io::Result<()>{
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Choose mode: (\"console\" for console input, \"file\" for file input, \"manual\" for manual function input)");

    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;

    if buffer.trim() == "console"{

        println!("Enter number of points:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *n = buffer.trim().parse().expect("Input is not Number");
        if *n<1 || *n>30 {panic!("Number either impossible or too big");}

        println!("Enter point:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *val = buffer.trim().parse().expect("Input is not Number");

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

        *val = line.trim().parse().expect("Input is not Number");

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
    else if buffer.trim() == "manual"
    {



        println!("Enter number of points:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *n = buffer.trim().parse().expect("Input is not Number");
        if *n<1 || *n>30 {panic!("Number either impossible or too big");}

        println!("Enter point:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *val = buffer.trim().parse().expect("Input is not Number");

        println!("Enter left boundary of interval:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        let a: f64 = buffer.trim().parse().expect("Input is not Number");

        println!("Enter right boundary of interval:");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        let b: f64 = buffer.trim().parse().expect("Input is not Number");
        if b<a {panic!("RIGHT boundary is bigger then LEFT!!");}

        *x = vec![0.0; *n];
        *y = vec![0.0; *n];

        for i in 0..*n {
            (*x)[i] = a + (i as f64)*((b-a)/(*n) as f64);
        }

        println!("Choose function:\n
        1) sinx\n
        2) e^x");
        let mut buffer = String::new();
        handle.read_line(&mut buffer)?;
        let chosen_function = buffer.trim().parse().expect("Input is not Number");
        match chosen_function {
            1 => for i in 0..*n {
                y[i] = x[i].sin();
            },
            2 => for i in 0..*n {
                y[i] = x[i].exp();
            },
            _ => panic!("Choose one of the following functions")
        };
        Ok(())
    }
    else {
        Err(Error::new(Other,"Unrecognisable input"))
    }

}


fn lagrange<'a>( n: usize, x : &'a Vec<f64>, y : &'a Vec<f64>) -> impl Fn(f64)->f64 + 'a{
    move |v| {let mut sum =0.0;
        for i in 0..n {
            sum+=y[i]*{let mut product = 1.0;
                for j in 0..n {
                    if j==i{continue;}
                    else{product*= (v-x[j])/(x[i] - x[j])}
                };
            product}
        };
    sum}
}

fn diff<'a>(k:usize,i:usize, x : &'a Vec<f64>, y : &'a Vec<f64>) -> f64
{
    if k==0 { y[i] }
    else { (diff(k-1,i+1,x,y) - diff(k-1,i,x,y))/(x[i+k] - x[i]) }
}

fn newton<'a>( n: usize, x : &'a Vec<f64>, y : &'a Vec<f64>) -> impl Fn(f64)->f64 + 'a{
    move |v| {let mut sum =y[0];
        for i in 1..n {
            sum+=diff(i,0,x,y)*{let mut product = 1.0;
                for j in 0..i {
                    product*=v-x[j];
                };
                product}
        };
        sum}
}

fn gauss<'a>( n: usize, x : &'a Vec<f64>, y : &'a Vec<f64>) -> impl Fn(f64)->f64 + 'a{
    let h: f64 = x[1] - x[0];
    let x0 = x[n/2];
        move |v| {let t  = (v-x0)/h;
            let mut sum = y[n/2];
            for i in 0..n {
                sum+=diff(i,(n-i)/2,x,y)*{
                    let mut product = t;
                    let mut delta = 0.0;
                    for j in 0..i {
                        product*=(t-delta*if j%2==0 {1.0} else{-1.0})/(if j!= 0 {j as f64} else{1.0}) ;
                        delta+=(j%2) as f64;
                    };
                    product}
            };
            sum}
}



fn draw_series<'a>(chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>, m:usize,method:METHOD, fun: impl Fn(f64) -> f64 + 'a, min_x:f64, max_x:f64, delta_x:f64) -> Result<(), Box<dyn std::error::Error>>{
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0).map(|x| (x,fun(x))),
                    match method {
                        LAGRANGE => &RED,
                        NEWTON => &BLUE,
                        GAUSS => &GREEN
                    },
                ))?
                .label(format!("polynomial {0}",m))
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n:usize = 0;
    let mut val:f64 = 0.0;
    let mut x:Vec<f64> = vec![];
    let mut y:Vec<f64> = vec![];


    match input(&mut n, &mut val, &mut x,&mut y) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };


    let root = BitMapBackend::new("out/interpolation.png", (WIDTH, HEIGHT)).into_drawing_area();

    let min_x = x.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_x = x.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_y = y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_y = y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let delta_x = ((max_x-min_x).abs())/100.0;
    let delta_y = ((max_y-min_y).abs())/100.0;


    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("My lovely graph on Rust", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((min_x-delta_x)..(max_x+delta_x), (min_y-delta_y)..(max_y+delta_y))?;

    chart.configure_mesh().draw()?;


    draw_series(&mut chart, n, LAGRANGE,lagrange(n,&x,&y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");
    draw_series(&mut chart, n, NEWTON,newton(n,&x,&y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");
    //draw_series(&mut chart, n, GAUSS,gauss(n,&x,&y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");

    println!("Largrange: {} \n", lagrange(n,&x,&y)(val) );
    println!("Newton: {} \n", newton(n,&x,&y)(val) );
    //println!("Gauss: {} \n", gauss(n,&x,&y)(val) );


    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}