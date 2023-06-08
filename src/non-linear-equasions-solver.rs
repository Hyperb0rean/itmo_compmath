use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use crate::Func::{Exponent, Linear, Polynomial, Sinus};
use crate::Method::{EqChord, EqIter, EqNewton, SysNewton};
use crate::SystemFunc::{Circle, Polynomials};


const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn polynom(x:f64) -> f64 {2.0*x.powi(3) - 9.0*x.powi(2) -7.0*x +11.0}
fn sinus(x:f64) -> f64 {x.sin()}
fn linear(x:f64) -> f64 {2.0*x}
fn exponent(x:f64) -> f64 {x.exp()}

fn dpolynom(x:f64) -> f64 {6.0*x.powi(2) - 18.0*x.powi(1) -7.0}
fn dsinus(x:f64) -> f64 {x.cos()}
fn dlinear(x:f64) -> f64 {2.0}
fn dexponent(x:f64) -> f64 {x.exp()}


#[derive(Clone,Copy)]
enum Func{
    Polynomial,
    Sinus,
    Linear,
    Exponent
}

#[derive(Clone,Copy)]
enum SystemFunc{
    Circle,
    Polynomials
}

#[derive(Clone,Copy)]
enum Method
{
    EqChord(Func),
    EqNewton(Func),
    EqIter(Func),
    SysNewton(SystemFunc)
}

fn chord(mut a:f64, mut b:f64, e:f64, func: Func) -> Result<f64,String>{
    let mut x =a;
    let f: fn(f64) ->f64 = match func { Polynomial => polynom, Sinus => sinus, Linear=> linear,Exponent=> exponent };
    let df: fn(f64) ->f64 = match func { Polynomial => dpolynom, Sinus => dsinus, Linear=> dlinear,Exponent=> dexponent };
    if f(a)*f(b) <= 0.0 {
        let mut counter: usize =0;
        while (a-b).abs() >=e && f(x).abs()>=e {
            if counter%2==0 {a= (a*f(b) - b*f(a))/(f(b) - f(a));} else {b= (a*f(b) - b*f(a))/(f(b) - f(a));}
            x = (a*f(b) - b*f(a))/(f(b) - f(a));
            counter+=1;
        }
        Ok(x)
    }
    else {Err("No root in this interval".to_string())}
}

fn newton(mut a:f64, mut b:f64, e:f64, func: Func) -> Result<f64,String>{
    let mut x =a;
    let f: fn(f64) ->f64 = match func { Polynomial => polynom, Sinus => sinus, Linear=> linear,Exponent=> exponent };
    let df: fn(f64) ->f64 = match func { Polynomial => dpolynom, Sinus => dsinus, Linear=> dlinear,Exponent=> dexponent };
    if f(a)*f(b) <= 0.0 {
        let mut x_old =b;
        while (x-x_old).abs() >=e && (f(x)).abs()>=e && ((f(x)/df(x))).abs() >= e {
            x_old=x;
            x = x - f(x)/df(x);
        }
        Ok(x)
    }
    else {Err("No root in this interval".to_string())}
}
fn iter(mut a:f64, mut b:f64, e:f64, func: Func) -> Result<f64,String>{
    let mut x =a;
    let f: fn(f64) ->f64 = match func { Polynomial => polynom, Sinus => sinus, Linear=> linear,Exponent=> exponent };
    let df: fn(f64) ->f64 = match func { Polynomial => dpolynom, Sinus => dsinus, Linear=> dlinear,Exponent=> dexponent };
    let phi = |x| x-f(x);
    let dphi = |x| 1.0-df(x);
    if f(a)*f(b) <= 0.0 {
        if dphi(x).abs() < 1.0 {
            let mut x_old = b;
            while (x - x_old).abs() >= e {
                x_old = x;
                x = phi(x);
            }
            Ok(x)
        }
        else { Err("Diverges!!".to_string()) }
    }
    else {Err("No root in this interval".to_string())}
}

fn input(e: &mut f64, a: &mut f64, b: &mut f64, method: &mut Method) -> io::Result<()> {

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Choose method:\n
    1) equation chords\n
    2) equation newton\n
    3) equation iteration\n
    4) system newton");
    let mut buffer = String::new();
    handle.read_line(&mut buffer)?;
    let chosen_method = buffer.trim().parse().expect("Input is not Number");
    match chosen_method
    {
        4=>{
            println!("Choose function:\n
            1) x^2 + y^2 = 4; y=x\n
            2) 2x^2 = 8; xy = 4;\n");
            buffer = String::new();
            handle.read_line(&mut buffer)?;
            let chosen_function = buffer.trim().parse().expect("Input is not Number");
            *method = match chosen_function { 1=> SysNewton(Circle), 2=>SysNewton(Polynomials), _=>panic!("Oops") }
        },
        1 | 2 | 3 =>{
            println!("Choose function:\n
            1) 2x^3 - 9x^2 -7x +11\n
            2) sin(x)\n
            3) 2x\n
            4) e^x");
            buffer = String::new();
            handle.read_line(&mut buffer)?;
            let chosen_function = buffer.trim().parse().expect("Input is not Number");
            match chosen_method {
                1 => *method = match chosen_function { 1=>  EqChord(Polynomial), 2=>  EqChord(Sinus), 3=> EqChord(Linear), 4=> EqChord(Exponent), _=>panic!("Ooops!")},
                2 => *method = match chosen_function { 1=>  EqNewton(Polynomial), 2=>  EqNewton(Sinus), 3=> EqNewton(Linear), 4=> EqNewton(Exponent), _=>panic!("Ooops!")},
                3 => *method = match chosen_function { 1=>  EqIter(Polynomial), 2=>  EqIter(Sinus), 3=> EqIter(Linear), 4=> EqIter(Exponent), _=>panic!("Ooops!")},
                _=>panic!("Ooops!")
            }
        }
        _ => println!("Choose one of the following!")
    }





    println!("Enter possible error:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *e = buffer.trim().parse().expect("Input is not Number");
    if *e <= 0.0 { panic!("Error should be bigger than 0");}

    println!("Enter lower boundary/ starting x:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *a = buffer.trim().parse().expect("Input is not Number");

    println!("Enter upper boundary/ starting y:");
    buffer = String::new();
    handle.read_line(&mut buffer)?;
    *b = buffer.trim().parse().expect("Input is not Number");

    Ok(())
}


fn draw_series<'a>(chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>, fun: impl Fn(f64) -> f64 + 'a, min_x:f64, max_x:f64, delta_x:f64) -> Result<(), Box<dyn std::error::Error>>{
    chart
        .draw_series(LineSeries::new(
            (((100.0*min_x).round() as i32)..=((100.0*max_x).round() as i32)).map(|x| x as f64/100.0).map(|x| (x,fun(x))),
            &RED,
        ))?
        .label("bebra")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut a = 0.0;
    let mut b = 0.0;
    let mut e=0.0;
    let mut method: Method = EqNewton(Linear);
    match input(&mut e, &mut a, &mut b, &mut method) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };

    let root = BitMapBackend::new("out/non-lin.png", (WIDTH, HEIGHT)).into_drawing_area();

    let min_x = -10.0;
    let max_x = 10.0;
    let min_y = -10.0;
    let max_y = 10.0;

    let delta_x = (max_x-min_x)/100.0;
    let delta_y = (max_y-min_y)/100.0;


    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("My lovely graph on Rust", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((min_x-delta_x)..(max_x+delta_x), (min_y-delta_y)..(max_y+delta_y))?;

    chart.configure_mesh().draw()?;


    println!("{}",match method {
        EqChord(func)=>{
            draw_series(&mut chart,match func { Polynomial => polynom, Sinus => sinus, Linear=> linear,Exponent=> exponent },min_x,max_x,delta_x).expect("Drawing go wrong!!!");
            match chord(a,b,e,func) { Ok(v) => v,Err(e) => panic!("{}",e)}
        },
        EqNewton(func)=>{
            draw_series(&mut chart,match func { Polynomial => polynom, Sinus => sinus, Linear=> linear,Exponent=> exponent },min_x,max_x,delta_x).expect("Drawing go wrong!!!");
            match newton(a,b,e,func) { Ok(v) => v,Err(e) => panic!("{}",e)}
        },
        EqIter(func)=>{
            draw_series(&mut chart,match func { Polynomial => polynom, Sinus => sinus, Linear=> linear,Exponent=> exponent },min_x,max_x,delta_x).expect("Drawing go wrong!!!");
            match iter(a,b,e,func) { Ok(v) => v,Err(e) => panic!("{}",e)}

        },
        SysNewton(sfunc)=>{
            10.0
        }
    });

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

