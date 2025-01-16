// makes the csv crate accessible to your program
extern crate csv;

use chrono::NaiveDate;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;
use rust_decimal::Decimal;

#[derive(Debug)]
enum StatementSource {
    Discover,
    Chase,
}

#[derive(Debug)]
struct Transaction {
    date: NaiveDate,

    description: String,

    amount: Decimal,

    category: String,

    source: StatementSource,

    // Optional fields for additional Chase data
    post_date: Option<NaiveDate>,

    transaction_type: Option<String>,

    memo: Option<String>,
}

// Trans. Date,Description,Amount,Category
#[derive(Debug, Deserialize)]
struct DiscoverRecord {
    #[serde(rename = "Trans. Date")]
    trans_date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Category")]
    category: String,
}

// Transaction Date,Post Date,Description,Category,Type,Amount,Memo
#[derive(Debug, Deserialize)]
struct ChaseRecord {
    #[serde(rename = "Transaction Date")]
    trans_date: String,

    #[serde(rename = "Post Date")]
    post_date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Category")]
    category: String,

    #[serde(rename = "Type")]
    trans_type: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Memo")]
    memo: String,
}

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

        println!(
            "Record {}: {} {} {}",
            trans_date, description, amount, category
        );

        // Process each record (row) from the CSV file
        //println!("{:?}", record);
    }

    Ok(())
}

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
// Trans. Date,Description,Amount,Category
fn read_discover_csv(filename: &str) -> Result<(), io::Error> {
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

        println!(
            "Record {}: {} {} {}",
            trans_date, description, amount, category
        );

        // Process each record (row) from the CSV file
        //println!("{:?}", record);
    }

    Ok(())
}

// chase statement
// Transaction Date,Post Date,Description,Category,Type,Amount,Memo
fn read_chase_csv(filename: &str) -> Result<(), io::Error> {
    let file = File::open(filename)?;

    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;

        let trans_date = &record[0];
        let post_date = &record[1];
        let description = &record[2];
        let category = &record[3];
        let trans_type = &record[4];
        let amount = &record[5];
        let memo = &record[6];

        println!(
            "Record {}: {} {} {}",
            trans_date, description, amount, category
        );

        println!("More Data: {}, {}, {}", post_date, trans_type, memo);

        // Process each record (row) from the CSV file
        //println!("{:?}", record);
    }

    Ok(())
}

fn new_read_csv_statement(filename: &str, source: StatementSource) -> Result<Vec<Transaction>, csv::Error>{
    
    let file = File::open(filename)?;

    let mut transactions = Vec::new();

    let mut reader = csv::Reader::from_reader(file);

    match source {
        
        StatementSource::Discover => {
            
            for result in reader.deserialize() {
                
                let record: DiscoverRecord = result?;

                // Parse and convert data into our unified Transaction structure
                let transaction = Transaction {
                    
                    date: NaiveDate::parse_from_str(&record.trans_date, "%Y-%m-%d")
                        .unwrap_or_default(),
                    
                    description: record.description,
                    
                    amount: record.amount.parse::<Decimal>().unwrap_or_default(),
                    
                    category: record.category,
                    
                    source: StatementSource::Discover,
                    
                    post_date: None,
                    
                    transaction_type: None,
                    
                    memo: None,

                };
                transactions.push(transaction);
            }
        }

        StatementSource::Chase => {
            
            for result in reader.deserialize() {
                
                let record: ChaseRecord = result?;

                // Parse and convert data into our unified Transaction structure
                let transaction = Transaction {
                    
                    date: NaiveDate::parse_from_str(&record.trans_date, "%m/%d/%Y")
                        .unwrap_or_default(),
                    
                    description: record.description,
                    
                    amount: record.amount.parse::<Decimal>().unwrap_or_default(),
                    
                    category: record.category,
                    
                    source: StatementSource::Chase,
                    
                    post_date: Some(
                        NaiveDate::parse_from_str(&record.post_date, "%m/%d/%Y")
                            .unwrap_or_default(),
                    ),
                    
                    transaction_type: Some(record.trans_type),
                    
                    memo: Some(record.memo),

                };
                transactions.push(transaction);
            }
        }

    }

    Ok(transactions)
}
