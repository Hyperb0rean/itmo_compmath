use std::arch::x86_64::_mm_extract_ps;
use std::cmp::{min, Ordering};
use std::f32::INFINITY;
use std::f64::NAN;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;
use std::num::FpCategory::Nan;
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, ChartContext};
use plotters::coord::Shift;
use plotters::coord::types::RangedCoordf64;
use plotters::drawing::{DrawingArea, DrawingAreaErrorKind, IntoDrawingArea};
use plotters::element::PathElement;
use plotters::prelude::{BLACK, Cartesian2d, Color, IntoFont, LineSeries, LogScalable, RED, WHITE};
use plotters::style::{BLUE, GREEN};
use plotters::style::full_palette::PURPLE;
use crate::METHOD::{GAUSS, LAGRANGE, NEWTON};

const ACCURACY: f64 =0.1;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[derive(Clone,Copy)]
enum METHOD{
    LAGRANGE,
    NEWTON,
    GAUSS
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
    let mut x:Vec<f64> = vec![];
    let mut y:Vec<f64> = vec![];


    match input(&mut n, &mut x,&mut y) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };


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
    draw_series(&mut chart, n, GAUSS,gauss(n,&x,&y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");


    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}