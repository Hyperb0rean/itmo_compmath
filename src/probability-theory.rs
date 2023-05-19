use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::io::ErrorKind::Other;
use std::ops::{ Range};
use plotters::prelude::*;



const ROUNDING:i32 = 3;

// /home/hyperb0rean/CLionProjects/itmo_compmath/input.txt
fn input(n: &mut usize, b: &mut Vec<f64>) -> io::Result<()>{
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


        *b = vec![0 as f64; *n];
        println!("Enter vector of free coefficients");
        buffer = String::new();
        handle.read_line(&mut buffer)?;
        *b = buffer.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*b).len() > *n { panic!("Vector is more than dimension") }

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

        *b = vec![0 as f64; *n];
        line = lines.next().unwrap()?;
        *b = line.trim().split(" ").map(|n| n.parse().expect("Input is not a Number")).collect();
        if (*b).len() > *n { panic!("Vector is more than dimension") }

        Ok(())
    }
    else {
        Err(Error::new(Other,"Unrecognisable input"))
    }

}

fn variation_series(x:&mut Vec<f64>) -> Vec<f64>{
    let mut new_vector: Vec<f64> = x.to_vec();
    new_vector.sort_by(|a, b| a.partial_cmp(b).unwrap());
    new_vector
}

fn standard_deviation(x:&mut Vec<f64>) -> f64{
    let mean: f64 = x.iter().sum::<f64>()/(x.len() as f64);
    let new_vector: Vec<f64> = x.to_vec();
    new_vector.iter().map(|i| (i-mean).powi(2) ).sum::<f64>()/((x.len()-1) as f64)
}

fn statistical_series(x:&mut Vec<f64>) -> (Vec<f64>, Vec<f64>){
    let mut values: Vec<f64> = Vec::new();
    let mut freqs: Vec<f64> = Vec::new();
    values.push(x[0]);
    freqs.push(1.0);
    for i in 1..x.len(){
        if x[i] == x[i-1] {
            let back =freqs.len()-1;
            freqs[back] += 1.0;
        }
        else {
            values.push(x[i]);
            freqs.push(1.0);
        }
    }
    (values,freqs.iter().map(|v| v/(x.len() as f64)).collect::<Vec<f64>>())
}

fn distribution_function(x:&mut (Vec<f64>,Vec<f64>)) -> (Vec<f64>,Vec<f64>){
    let  values: Vec<f64> = x.to_owned().0;
    let mut freqs: Vec<f64> = x.to_owned().1;
    let mut sum = 0.0;
    for i in 0..freqs.len(){
        sum+=freqs[i];
        freqs[i]=sum;
    }
    (values,freqs.iter().map(|v| (v*10.0_f64.powi(ROUNDING)).round()/10.0_f64.powi(ROUNDING)).collect())
}

fn histogram_series(x:&mut Vec<f64>, h: f64) -> Vec<f64>{
    let mut new_vector: Vec<f64> = x.to_vec();
    new_vector.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut prev = new_vector[0] - h/2.0;
    let mut next = prev +h;
    for i in 0..new_vector.len() {
        if new_vector[i] > next || new_vector[i] < prev {
            prev =next;
            next = prev+h;
        }
        new_vector[i] = (prev + next)/2.0
    }
    new_vector
}


fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let mut n:usize = 0;
    let mut x:Vec<f64> = vec![];
    match input(&mut n,&mut x) { Ok(()) => (),Err(e) => panic!("{}",e.to_string()) };

    println!("Variation series: {:?}",variation_series(&mut x));
    println!("Extreme values: {}, {}", variation_series(&mut x)[0],variation_series(&mut x)[n-1]);
    println!("Scope: {}",variation_series(&mut x)[n-1] - variation_series(&mut x)[0]);
    println!("Mean: {}", x.iter().sum::<f64>()/(n as f64));
    println!("Standard deviation: {}", standard_deviation(&mut x));
    println!("Statistical series: {:?}", statistical_series(&mut variation_series(&mut x)));

    let  values: Vec<f64> = distribution_function(&mut statistical_series(&mut variation_series(&mut x))).0;
    let  freqs: Vec<f64> = distribution_function(&mut statistical_series(&mut variation_series(&mut x))).1;

    let root = BitMapBackend::new("out/dist.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Distribution function", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d::<Range<f64>,Range<f64>>(variation_series(&mut x)[0]..variation_series(&mut x)[n-1], 0.0..1.0)?;

    chart.configure_mesh().draw()?;

    for i in 1..values.len() {
        chart
            .draw_series(LineSeries::new([(values[i-1],freqs[i-1]), (values[i]-0.001,freqs[i-1])],
                                         RED.filled(),
            ).point_size(1))?;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    let statistical_series = statistical_series(&mut variation_series(&mut x));

    let root = BitMapBackend::new("out/poly.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Polygon", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d::<Range<f64>,Range<f64>>(variation_series(&mut x)[0]..variation_series(&mut x)[n-1], 0.0..1.0)?;

    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new((0..statistical_series.0.len()).map(|i| (statistical_series.0[i], statistical_series.1[i])),
                                     RED.filled(),
        ).point_size(1))?;


    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;


    let mut var = variation_series(&mut x);
    let h = (var[n-1] - var[0])/(1. + (n as f64).log2());
    let data = histogram_series(&mut x,h);

    println!("Hist series: {:?}", data);

    let root = BitMapBackend::new("out/hist.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d(((data[0])..(data[n-1]+h/4.)).step(h).use_round().into_segmented(), 0u32..(n as u32))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;



    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x| (x.to_owned(), 1))),
    )?;


    Ok(())
}