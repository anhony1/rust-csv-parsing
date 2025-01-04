use plotters::prelude::*;

// makes the csv crate accessible to your program
extern crate csv;

use std::fs::File;

use std::error::Error;
use std::io;
use std::process;

fn main() {
    
    draw_graph().unwrap();

    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }

}

fn run() -> Result<(), Box<dyn Error>> {
    
    let absolute_file_path = "";

    // The question mark is the error propagation operator in Rust. When placed after
    // a Result, it either returns the value inside the Ok variant, or it returns the
    // error from the Err variant. This is a convenient way to handle errors in Rust.

    read_csv(absolute_file_path)?;

    // ok() is a method that creates a Result with the Ok variant. 

    Ok(())

}

// Discover Statement
// Trans. Date,Description,Amount,Category

// Chase Statement
// Transaction Date,Post Date,Description,Category,Type,Amount,Memo


fn read_csv(filename: &str) -> Result<(), io::Error> {

    let file = File::open(filename)?; 

    let mut counter = 0;

    let mut reader = csv::Reader::from_reader(file); 

    for result in reader.records() {

        counter += 1;

        let record = result?; 

        let trans_date = &record[0];
        let description = &record[1];
        let amount = &record[2];
        let category = &record[3];

        println!("Record {}: {} {} {}", trans_date, description, amount, category);

        // Process each record (row) from the CSV file
        //println!("{:?}", record);

    }


    Ok(())

}

// We are going to have a couple graphs that we want to use to visual our data and then display it onto SLINT UI...

// generating png images... gunna have to figure that one out


fn draw_graph() -> Result<(), Box<dyn std::error::Error>> {
    


    let root = BitMapBackend::new("plotters-doc-data/1.png", (640, 480)).into_drawing_area();
    
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^3", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

// discover statement


// chase statement


// sams club statement

