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
use crate::FUNCTION::{EXPONENTIAL, LOGARITHMIC, POLYNOMIAL, POWER};

const ACCURACY: f64 =0.001;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[derive(Clone, Copy)]
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
    match f {
        FUNCTION::POLYNOMIAL(m)=>{
            let mut b:Vec<f64> = vec![0 as f64; (m+1) as usize];
            let mut matrix:Vec<Vec<f64>> = vec![vec![0 as f64; (m+1) as usize]; (m+1) as usize];

            for i in 0..=m {
                for j in 0..n{
                    b[i as usize]+= ((*x)[j as usize].powi(i as i32))*(*y)[j as usize];
                }
            }

            for i in 0..=m {
                for j in 0..=m{
                    matrix[i as usize][j as usize] = x.iter().map(|v| v.powi((i + j) as i32)).sum();
                }
            }
            linear_calculation((m+1) as usize, &mut matrix, &mut b, ACCURACY)
        }
        FUNCTION::EXPONENTIAL => {
            let mut a = approximation_calculation(POLYNOMIAL(1),n,x,&((*y).iter().map(|v| v.ln()).collect()));
            a[0] = a[0].exp();
            a
        }
        FUNCTION::LOGARITHMIC => {
            approximation_calculation(POLYNOMIAL(1),n,&((*x).iter().map(|v| v.ln()).collect()),y)
        }
        FUNCTION::POWER =>{
            approximation_calculation(POLYNOMIAL(1),n,
                                              &((*x).iter().map(|v| v.ln()).collect()),&((*y).iter().map(|v| v.ln()).collect()))
        }
    }

}

fn linear_calculation(n: usize,a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>, e: f64) -> Vec<f64>{
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
        if delta < e { break; }
    }
    v_x
}


fn find_best_function(n: usize,x : &Vec<f64>, y : &Vec<f64>) -> FUNCTION {
    let mut deviations: Vec<(f64,FUNCTION)> = vec![(0.0,FUNCTION::POLYNOMIAL(1));6];
    for i in 1..=3 {
        deviations[i-1] = (standart_deviation_calculation(POLYNOMIAL(i as u16),n,
                                                      &differences_calculation(POLYNOMIAL(i as u16), n,
                                                      &approximation_calculation(POLYNOMIAL(i as u16),n,&x,&y),&x, &y)), POLYNOMIAL(i as u16));
    }
    deviations[3] =  (standart_deviation_calculation(EXPONENTIAL,n,
                                                  &differences_calculation(EXPONENTIAL, n,
                                                                           &approximation_calculation(EXPONENTIAL,n,&x,&y),&x, &y)),EXPONENTIAL);
    deviations[4] =  (standart_deviation_calculation(LOGARITHMIC,n,
                                                     &differences_calculation(LOGARITHMIC, n,
                                                                              &approximation_calculation(LOGARITHMIC,n,&x,&y),&x, &y)),LOGARITHMIC);
    deviations[5] =  (standart_deviation_calculation(POWER,n,
                                                     &differences_calculation(POWER, n,
                                                                              &approximation_calculation(POWER,n,&x,&y),&x, &y)),POWER);
    deviations.sort_by(|a,b|  (a.0).partial_cmp(&(b.0)).unwrap_or(Ordering::Greater));
    deviations[0].1
}

fn standart_deviation_calculation(f: FUNCTION, n: usize, e : &Vec<f64>) -> f64{
    let mut standard_deviation: f64 = 0.0;
    for i in 0..n{
        standard_deviation+=(e[i]).powi(2);
    }
    standard_deviation/=n as f64;
    standard_deviation = standard_deviation.sqrt();
    standard_deviation
}

fn differences_calculation(f: FUNCTION, n: usize, a : &Vec<f64>, x : &Vec<f64>, y : &Vec<f64>) -> Vec<f64>{
    let mut differences: Vec<f64> = vec![0.0;n];
    for i in 0..n{
        differences[i] = get_function(f,a,x[i]) - y[i];
    }
    differences
}

fn get_function(f: FUNCTION, a : &Vec<f64>,x:f64) ->  f64{
    match f {
        FUNCTION::POLYNOMIAL(m)=>{
            {
                let mut sum :f64 = 0.0;
                for i in 0..=m {
                    sum+= a[i as usize]*(x.powi(i as i32));
                }
                sum
            }
        }
        FUNCTION::EXPONENTIAL => {
            a[0]*(((x as f64)*a[1] as f64).exp())
        }
        FUNCTION::LOGARITHMIC => {
            a[0]*((x).ln()) + a[1]
        }
        FUNCTION::POWER =>{
            a[0] * (x.powf(a[1]))
        }
    }
}


fn print_function(f: FUNCTION, a :&Vec<f64>){
    println!("{}",match f {
        FUNCTION::POLYNOMIAL(m)=>{
            {
                let mut string : String = Default::default();
                for i in 0..=m {
                    string+= &*a[i as usize].to_string();
                    if i!=0 {
                        string+="x^";
                        string+= &*(i).to_string();
                    }
                    if i!= m {
                        string+= " + ";
                    }
                }
                string
            }
        }
        FUNCTION::EXPONENTIAL => {
            a[0].to_string() + "e^" + &*a[1].to_string() + "x"
        }
        FUNCTION::LOGARITHMIC => {
            a[0].to_string() + "lnx" +" + "+  &*a[1].to_string()
        }
        FUNCTION::POWER =>{
            a[0].to_string() + "x^" + &*a[1].to_string()
        }
    });
}

fn draw_series(chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>, f:FUNCTION, a:&Vec<f64>, min_x:f64, max_x:f64, delta_x:f64) -> Result<(), Box<dyn std::error::Error>>{
    match f {
        FUNCTION::POLYNOMIAL(m) => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0).map(|x| (x,get_function(POLYNOMIAL(m),a,x))),
                    &RED,
                ))?
                .label(format!("polynomial {0}",m))
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        }

        FUNCTION::EXPONENTIAL => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0)
                        .map(|x| (x,get_function(EXPONENTIAL,a,x))),
                    &BLUE,
                ))?
                .label("exponential")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        }
        FUNCTION::LOGARITHMIC => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0)
                        .map(|x| (x,get_function(LOGARITHMIC,a,x))),
                    &GREEN,
                ))?
                .label("logarithmic")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
        }
        FUNCTION::POWER => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0)
                        .map(|x| (x,get_function(POWER,a,x))),
                    &PURPLE,
                ))?
                .label("power")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &PURPLE));
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n:usize = 0;
    let mut x:Vec<f64> = vec![];
    let mut y:Vec<f64> = vec![];

    match input(&mut n, &mut x,&mut y) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };

    print_function(    find_best_function(n,&x,&y),&approximation_calculation(find_best_function(n,&x,&y),n,&x,&y));

    let root = BitMapBackend::new("out/approximation.png", (WIDTH, HEIGHT)).into_drawing_area();

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


    draw_series(&mut chart, POLYNOMIAL(1), &approximation_calculation(POLYNOMIAL(1), n, &mut x, &mut y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");
    draw_series(&mut chart, POLYNOMIAL(2), &approximation_calculation(POLYNOMIAL(2), n, &mut x, &mut y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");
    draw_series(&mut chart, POLYNOMIAL(3), &approximation_calculation(POLYNOMIAL(3), n, &mut x, &mut y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");
    draw_series(&mut chart, EXPONENTIAL, &approximation_calculation(EXPONENTIAL, n, &mut x, &mut y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");
    draw_series(&mut chart, LOGARITHMIC, &approximation_calculation(LOGARITHMIC, n, &mut x, &mut y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");
    draw_series(&mut chart, POWER, &approximation_calculation(POWER, n, &mut x, &mut y), min_x, max_x, delta_x).expect("Drawing go wrong!!!");

    println!("{:?}",&approximation_calculation(POWER,n,&x,&y));
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}