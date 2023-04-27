use std::arch::x86_64::_mm_extract_ps;
use std::cmp::min;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, ChartContext};
use plotters::coord::Shift;
use plotters::coord::types::RangedCoordf64;
use plotters::drawing::{DrawingArea, DrawingAreaErrorKind, IntoDrawingArea};
use plotters::element::PathElement;
use plotters::prelude::{BLACK, Cartesian2d, Color, IntoFont, LineSeries, RED, WHITE};
use crate::FUNCTION::{EXPONENTIAL, POLYNOMIAL};

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
            calculation((m+1) as usize, &mut matrix, &mut b, ACCURACY)
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

fn calculation(n: usize,a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>, e: f64) -> Vec<f64>{
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

// fn draw_series(root:&DrawingArea<BitMapBackend,Shift>, chart: &mut Result<ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>, DrawingAreaErrorKind<plotters_bitmap::BitMapBackendError>>, f:FUNCTION, a:&Vec<f64>, x:&Vec<f64>, y:&Vec<f64>) -> Result<(), Box<dyn std::error::Error>>{
//
//     let min_x = x.iter().fold(f64::INFINITY, |a, &b| a.min(b));
//     let max_x = x.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
//
//     match f {
//         FUNCTION::POLYNOMIAL(m) => {
//             chart.unwrap()
//                 .draw_series(LineSeries::new(
//                     ((min_x.round() as i32)..=(max_x.round() as i32)).map(|x| x as f64 ).map(|x| (x,{
//                         let mut sum :f64 = 0.0;
//                         for i in 1..=m {
//                             sum+= a[(i-1) as usize]*(x.powi(i as i32));
//                         }
//                         sum
//                     })),
//                     &RED,
//                 ))?
//                 .label("polynomial")
//                 .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
//         }
//         FUNCTION::EXPONENTIAL => {
//             (*chart)?
//                 .draw_series(LineSeries::new(
//                     ((min_x.round() as i32)..=(max_x.round() as i32)).map(|x| x as f64 ).map(|x| (x,a[0]*(((x as f64)*a[1] as f64).exp()))),
//                     &RED,
//                 ))?
//                 .label("e^x")
//                 .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
//         }
//         FUNCTION::LOGARITHMIC => {
//             (*chart)?
//                 .draw_series(LineSeries::new(
//                     ((min_x.round() as i32)..=(max_x.round() as i32)).map(|x| x as f64 ).map(|x| (x,a[0]*((x).ln()) + a[1])),
//                     &RED,
//                 ))?
//                 .label("lnx")
//                 .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
//         }
//         FUNCTION::POWER => {
//             (*chart)?
//                 .draw_series(LineSeries::new(
//                     ((min_x.round() as i32)..=(max_x.round() as i32)).map(|x| x as f64 ).map(|x| (x,a[0]*((x).powf(a[1])))),
//                     &RED,
//                 ))?
//                 .label("x^a")
//                 .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
//         }
//     }
//     Ok(())
// }

// fn draw_chart<'a>(root:&'a DrawingArea<BitMapBackend<'a>,Shift>, x:&'a Vec<f64>,y:&'a Vec<f64>)
//     -> Result<ChartContext<'a, BitMapBackend<'a>,Cartesian2d<RangedCoordf64,RangedCoordf64>>, Box<dyn std::error::Error>>{
//     let min_x = x.iter().fold(f64::INFINITY, |a, &b| a.min(b));
//     let max_x = x.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
//     let min_y = y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
//     let max_y = y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
//
//     let delta = ((max_x-min_x).abs() + (max_y-min_y).abs())/1000.0;
//
//     (*root).fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(root)
//         .caption("My awesome graph in Rust", ("sans-serif", 50).into_font())
//         .margin(5)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d((min_x-delta)..(max_x+delta), (min_y-delta)..(max_y+delta));
//
//     chart?.configure_mesh().draw()?;
//
//     Ok(chart.unwrap())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n:usize = 0;
    let mut x:Vec<f64> = vec![];
    let mut y:Vec<f64> = vec![];

    match input(&mut n, &mut x,&mut y) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };



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

    let f : FUNCTION = EXPONENTIAL;

    let a=approximation_calculation(f,n,&mut x, &mut y);

    println!("Your approximation coefficients: {:?}",&a);
    match f {
        FUNCTION::POLYNOMIAL(m) => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0).map(|x| (x,{
                        let mut sum :f64 = 0.0;
                        for i in 0..=m {
                            sum+= a[i as usize]*(x.powi(i as i32));
                        }
                        sum
                    })),
                    &RED,
                ))?
                .label("polynomial")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        }
        FUNCTION::EXPONENTIAL => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0)
                        .map(|x| (x,a[0]*(((x as f64)*a[1] as f64).exp()))),
                    &RED,
                ))?
                .label("e^x")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        }
        FUNCTION::LOGARITHMIC => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0)
                        .map(|x| (x,a[0]*((x).ln()) + a[1])),
                    &RED,
                ))?
                .label("lnx")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        }
        FUNCTION::POWER => {
            chart
                .draw_series(LineSeries::new(
                    (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0)
                        .map(|x| (x,a[0]*((x).powf(a[1])))),
                    &RED,
                ))?
                .label("x^a")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        }
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}